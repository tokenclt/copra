use bytes::Bytes;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::multiplex::{ClientProto, ClientService};
use tokio_proto::TcpClient;
use tokio_service::Service;
use futures::{Async, Future, IntoFuture, Poll, Stream};
use futures::sync::mpsc;
use futures::sync::oneshot;
use std::io;
use std::net::{AddrParseError, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use protocol::{BrpcProtocol, ProtoCodecClient, Protocol, RpcProtocol};
use load_balancer::single_server::SingleServerLoadBalancer;
use message::{RpcRequestMeta, RpcResponseMeta};
use self::transport::Transport;
use self::backend::ChannelBackend;
use self::connector::Connector;

pub mod backend;
pub mod connector;
pub mod transport;

type RequestPackage = (RpcRequestMeta, Bytes);
type ResponsePackage = (RpcResponseMeta, Bytes);

pub type ChannelBuildFuture = Box<Future<Item = Channel, Error = ChannelBuildError>>;

//type ConcreteClientService = ClientService<TcpStream, MetaClientProtocol>;

type OneShotSender = oneshot::Sender<io::Result<ResponsePackage>>;

type OneShotReceiver = oneshot::Receiver<io::Result<ResponsePackage>>;

type ChannelSender = mpsc::UnboundedSender<(OneShotSender, RequestPackage)>;

type ChannelReceiver = mpsc::UnboundedReceiver<(OneShotSender, RequestPackage)>;

#[derive(Clone, Debug)]
pub enum ChannelBuildError {
    AddrParseError,
    ConnectError,
}

#[derive(Debug)]
pub enum ChannelError {
    ConcurrencyLimitReached,
    IoError(io::Error),
    UnknownError,
}

impl From<AddrParseError> for ChannelBuildError {
    fn from(_: AddrParseError) -> Self {
        ChannelBuildError::AddrParseError
    }
}

pub struct MetaClientProtocol {
    proto: Box<RpcProtocol>,
    handle: Handle,
    addr: SocketAddr,
}

impl MetaClientProtocol {
    pub fn new(proto_type: &Protocol, handle: Handle, addr: SocketAddr) -> Self {
        let proto = match proto_type {
            // TODO: unify construction interface of protocols
            &Protocol::Brpc => Box::new(BrpcProtocol::new()),
            _ => unimplemented!(),
        };
        MetaClientProtocol {
            proto,
            handle,
            addr,
        }
    }
}

impl ClientProto<TcpStream> for MetaClientProtocol {
    type Request = RequestPackage;
    type Response = ResponsePackage;
    type Transport = Transport<Framed<Connector, ProtoCodecClient>>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: TcpStream) -> Self::BindTransport {
        let conn = Connector::from_stream(self.addr.clone(), io, self.handle.clone());
        let codec = ProtoCodecClient::new(self.proto.new_boxed());
        let transport = Transport::new(conn.framed(codec));
        Ok(transport)
    }
}

#[derive(Debug)]
enum ConnectMode {
    Single(&'static str),
}

pub struct ChannelBuilder {
    mode: ConnectMode,
    handle: Handle,
    protocol: Option<Protocol>,
    deadline: Option<Option<Duration>>,
    max_retry: Option<u32>,
    max_concurrency: Option<u32>,
}

impl ChannelBuilder {
    pub fn single_server(addr: &'static str, handle: Handle) -> Self {
        ChannelBuilder {
            mode: ConnectMode::Single(addr),
            handle: handle,
            protocol: None,
            deadline: None,
            max_retry: None,
            max_concurrency: None,
        }
    }

    pub fn build(self) -> ChannelBuildFuture {
        let protocol = self.protocol.unwrap_or(Protocol::Brpc);
        let deadline = self.deadline.unwrap_or(None);
        let max_retry = self.max_retry.unwrap_or(3);
        let max_concurrency = self.max_concurrency.unwrap_or(1_000_000);
        let handle = self.handle;

        let (tx, rx) = mpsc::unbounded();
        let channel = Channel::new(tx, max_concurrency);

        match self.mode {
            ConnectMode::Single(addr) => {
                let parse = addr.parse::<SocketAddr>()
                    .map_err(|_| ChannelBuildError::AddrParseError)
                    .into_future();
                let fut = parse.and_then(move |addr| {
                    let proto = MetaClientProtocol::new(&protocol, handle.clone(), addr.clone());
                    TcpClient::new(proto)
                        .connect(&addr, &handle)
                        .map_err(|_| ChannelBuildError::ConnectError)
                        .map(move |end_port| {
                            let lb = SingleServerLoadBalancer::new(end_port);
                            let backend = ChannelBackend::new(rx, handle.clone(), lb);
                            handle.spawn(backend);
                            channel
                        })
                });
                Box::new(fut)
            }
        }
    }
}

pub struct ChannelFuture {
    rx: Option<OneShotReceiver>,
    counter: Arc<AtomicUsize>,
}

impl ChannelFuture {
    pub fn new(rx: Option<OneShotReceiver>, counter: Arc<AtomicUsize>) -> Self {
        ChannelFuture { rx, counter }
    }
}

impl Future for ChannelFuture {
    type Item = ResponsePackage;

    type Error = ChannelError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if let Some(ref mut rx) = self.rx {
            let result = try_ready!(
                rx.poll()
                    .map_err(|_| panic!("The sending end of the oneshot is dropped"))
            );
            self.counter.fetch_sub(1, Ordering::Relaxed);

            result
                .map_err(|e| ChannelError::IoError(e))
                .map(|resp| Async::Ready(resp))
        } else {
            Err(ChannelError::ConcurrencyLimitReached)
        }
    }
}

#[derive(Clone)]
pub struct Channel {
    sender: ChannelSender,
    counter: Arc<AtomicUsize>,
    max_concurrency: usize,
}

impl Channel {
    pub fn new(sender: ChannelSender, max_concurrency: u32) -> Self {
        Channel {
            sender,
            counter: Arc::new(AtomicUsize::new(0)),
            max_concurrency: max_concurrency as usize,
        }
    }

    pub fn call(&self, req: RequestPackage) -> ChannelFuture {
        let (tx, rx) = oneshot::channel();
        let rx = if self.counter.load(Ordering::Relaxed) < self.max_concurrency {
            self.counter.fetch_add(1, Ordering::Relaxed);
            self.sender
                .unbounded_send((tx, req))
                .expect("The receiving end is dropped");
            Some(rx)
        } else {
            None
        };

        ChannelFuture::new(rx, self.counter.clone())
    }
}
