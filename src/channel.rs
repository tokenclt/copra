use bytes::BytesMut;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::multiplex::{ClientProto, ClientService};
use tokio_proto::TcpClient;
use tokio_service::Service;
use futures::{Async, Future, IntoFuture, Poll, Sink, Stream};
use futures::sync::mpsc;
use futures::sync::oneshot;
use std::io;
use std::clone;
use std::marker::PhantomData;
use std::net::{AddrParseError, SocketAddr};
use std::time::Duration;

use protocol::{BrpcProtocol, Meta, ProtoCodec, Protocol, RpcProtocol};
use service::MethodError;

pub type ChannelFuture = Box<Future<Item = Meta, Error = MethodError>>;

pub type ConnectFuture<S> = Box<
    Future<Item = (Channel, ChannelBackend<S>), Error = ChannelInitError>,
>;

type ConcreteClientService = ClientService<TcpStream, MetaClientProtocol>;

type OneShotSender = oneshot::Sender<Result<Meta, MethodError>>;

type ChannelSender = mpsc::UnboundedSender<(OneShotSender, Meta)>;

type ChannelReceiver = mpsc::UnboundedReceiver<(OneShotSender, Meta)>;


pub enum ChannelInitError {
    AddrParseError,
    UnknownError,
}

impl From<AddrParseError> for ChannelInitError {
    fn from(_: AddrParseError) -> Self {
        ChannelInitError::AddrParseError
    }
}

#[derive(Clone, Debug)]
pub struct ChannelOption {
    pub protocol: Protocol,
    pub deadline: Duration,
    pub max_retry: u32,
}

pub struct MetaClientProtocol {
    proto: Box<RpcProtocol>,
}

impl MetaClientProtocol {
    pub fn new(option: &ChannelOption) -> Self {
        let proto = match option.protocol {
            // TODO: unify construction interface of protocols
            Protocol::Brpc => Box::new(BrpcProtocol),
            _ => unimplemented!(),
        };
        MetaClientProtocol { proto }
    }
}

impl<T> ClientProto<T> for MetaClientProtocol
where
    T: AsyncRead + AsyncWrite + 'static,
{
    type Request = Meta;
    type Response = Meta;
    type Transport = Framed<T, ProtoCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let codec = ProtoCodec::with_protocol(self.proto.box_clone());
        Ok(io.framed(codec))
    }
}

pub struct ChannelBuilder<S> {
    phantom: PhantomData<S>,
}

impl<S> ChannelBuilder<S> {
    /// Connect to the server at X.X.X.X:port
    pub fn single_server(
        addr: &str,
        handle: Handle,
        option: ChannelOption,
    ) -> ConnectFuture<ConcreteClientService> {
        let socket_addr = match addr.parse() {
            Ok(a) => a,
            Err(_) => {
                let err = Err(ChannelInitError::AddrParseError);
                return Box::new(err.into_future());
            }
        };
        let (tx, rx) = mpsc::unbounded();
        let channel = Channel::new(tx);

        let fut = TcpClient::new(MetaClientProtocol::new(&option))
            .connect(&socket_addr, &handle)
            .map_err(|_| ChannelInitError::UnknownError)
            .map(|service| {
                (channel, ChannelBackend::new(option, rx, handle, service))
            });

        Box::new(fut)
    }
}

#[derive(Clone)]
pub struct Channel {
    sender: ChannelSender,
}

impl Channel {
    pub fn new(sender: ChannelSender) -> Self {
        Channel { sender }
    }

    pub fn call(&self, req: Meta) -> ChannelFuture {
        let (tx, rx) = oneshot::channel();
        let fut = self.sender
            .unbounded_send((tx, req))
            .map_err(|_| panic!("The receiving end of the mpsc is dropped."))
            .into_future()
            .and_then(|_| rx)
            // TODO: maybe ignore this.
            // refering to request cancelation.
            .map_err(|_| panic!("The sending end of the oneshot is dropped"))
            .and_then(|result| result);
        Box::new(fut)
    }
}

pub struct ChannelBackend<S> {
    option: ChannelOption,
    recv: ChannelReceiver,
    handle: Handle,
    abstract_service: S,
}

impl<S> ChannelBackend<S> {
    pub fn new(
        option: ChannelOption,
        recv: ChannelReceiver,
        handle: Handle,
        abstract_service: S,
    ) -> Self {
        ChannelBackend {
            option,
            recv,
            handle,
            abstract_service,
        }
    }
}

impl<S> ChannelBackend<S>
where
    S: Service<Request = Meta, Response = Meta, Error = io::Error>,
    S: 'static,
{
    fn spawn(&mut self, sender: OneShotSender, meta: Meta) {
        let fut = self.abstract_service
            .call(meta)
            // TODO: fill in a meaningful error
            .map_err(|e| MethodError::UnknownError)
            .then(|result| sender.send(result))
            // TODO: Or maybe just ignore this error, for the rpc request might be cancelled.
            .map_err(|_| panic!("The receiving end of the oneshot is dropped."));
        self.handle.spawn(fut);
    }
}

impl<S> Future for ChannelBackend<S>
where
    S: Service<Request = Meta, Response = Meta, Error = io::Error>,
    S: 'static,
{
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            match try_ready!(self.recv.poll()) {
                Some((sender, meta)) => self.spawn(sender, meta),
                None => return Ok(Async::Ready(())),
            }
        }
    }
}
