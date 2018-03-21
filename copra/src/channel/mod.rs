//! Communication channel between servers
//!
//! This module contains the `Channel` struct and some types interact with it.
//!
//! # Examples
//!
//! ```no_run
//! # extern crate copra;
//! # extern crate tokio_core;
//! # use std::error::Error;
//! use copra::ChannelBuilder;
//! use tokio_core::reactor::Core;
//!
//! # fn main() {
//! #     try_main().unwrap();
//! # }
//! # fn try_main() -> Result<(), Box<Error>> {
//! let mut core = Core::new()?;
//! let builder = ChannelBuilder::single_server("127.0.0.1:8000", core.handle());
//! let channel = core.run(builder.build())?;
//! # Ok(())
//! # }
//! ```

use bytes::Bytes;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_io::AsyncRead;
use tokio_io::codec::Framed;
use tokio_proto::multiplex::ClientProto;
use tokio_proto::TcpClient;
use futures::{Async, Future, IntoFuture, Poll};
use futures::sync::mpsc;
use futures::sync::oneshot;
use std::error::Error;
use std::fmt;
use std::io;
use std::net::{AddrParseError, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use protocol::{BrpcProtocol, ProtoCodecClient, Protocol, RpcProtocol};
use load_balancer::{CallInfo, ServerEndPort, ServerId};
use load_balancer::single_server::SingleServerLoadBalancer;
use message::{RpcRequestMeta, RpcResponseMeta};

use self::backend::ChannelBackend;
use self::connector::Connector;

mod backend;
pub(crate) mod connector;
mod layer;

/// A future returned by `ChannelBuilder::build` which will resolve to a `Channel`
/// when the channel is ready for use.
pub type ChannelBuildFuture = Box<Future<Item = Channel, Error = ChannelBuildError>>;

// FIXME: name conflicts with message::*
// TODO: make this fully public
// TODO: move this to a better place
pub(crate) type RequestPackage = (RpcRequestMeta, Bytes);

pub(crate) type ResponsePackage = (RpcResponseMeta, Bytes);

type FeedbackSender = oneshot::Sender<(ServerId, CallInfo)>;

type FeedbackReceiver = oneshot::Receiver<(ServerId, CallInfo)>;

type OneShotSender = oneshot::Sender<io::Result<(ResponsePackage, FeedbackHandle)>>;

type OneShotReceiver = oneshot::Receiver<io::Result<(ResponsePackage, FeedbackHandle)>>;

type ChannelSender = mpsc::UnboundedSender<(OneShotSender, RequestPackage)>;

type ChannelReceiver = mpsc::UnboundedReceiver<(OneShotSender, RequestPackage)>;

/// The error when building a channel
#[derive(Clone, Debug)]
pub enum ChannelBuildError {
    /// An error occured when parsing `SocketAddr` from string
    AddrParseError(AddrParseError),
    /// Failed to connect to a server or a cluster
    ConnectError,
}

impl fmt::Display for ChannelBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChannelBuildError::AddrParseError(ref e) => write!(f, "address parse error: {}", e),
            ChannelBuildError::ConnectError => write!(f, "connection error"),
        }
    }
}

impl Error for ChannelBuildError {
    fn description(&self) -> &str {
        match *self {
            ChannelBuildError::AddrParseError(_) => "failed to parse socket address from raw string",
            ChannelBuildError::ConnectError => "failed to connect to a remote server",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ChannelBuildError::AddrParseError(ref e) => Some(e),
            ChannelBuildError::ConnectError => None,
        }
    }
}

/// [WIP] The error occured when sending or receiving packages
#[derive(Debug)]
pub enum ChannelError {
    /// Can not issue new request because the number of pending requests has
    /// reached the concurrency limit
    ConcurrencyLimitReached,
    /// Io error from TCP socket
    IoError(io::Error),
    /// [WIP] Other errors that need to be explicated
    UnknownError,
}

impl fmt::Display for ChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChannelError::ConcurrencyLimitReached => write!(f, "Concurrency limit reached"),
            ChannelError::IoError(ref e) => write!(f, "Io error: {}", e),
            ChannelError::UnknownError => write!(f, "other errors might be worth discussion"),
        }
    }
}

impl Error for ChannelError {
    fn description(&self) -> &str {
        match *self {
            ChannelError::ConcurrencyLimitReached => "concurrency limit reached",
            ChannelError::IoError(_) => "io error from TCP socket",
            ChannelError::UnknownError => "[WIP] other errors",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ChannelError::IoError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<AddrParseError> for ChannelBuildError {
    fn from(e: AddrParseError) -> Self {
        ChannelBuildError::AddrParseError(e)
    }
}

//TODO: make this private
#[doc(hidden)]
pub struct MetaClientProtocol {
    proto: Box<RpcProtocol>,
    handle: Handle,
    addr: SocketAddr,
}

impl fmt::Debug for MetaClientProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MetaClientProtocol {{ addr: {:?} }}", self.addr)
    }
}

impl MetaClientProtocol {
    /// Create a new instance.
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
    type Transport = Framed<Connector, ProtoCodecClient>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: TcpStream) -> Self::BindTransport {
        let conn = Connector::from_stream(self.addr.clone(), io, self.handle.clone());
        let codec = ProtoCodecClient::new(self.proto.new_boxed());
        let framed = conn.framed(codec);
        Ok(framed)
    }
}

#[derive(Debug)]
enum ConnectMode<'a> {
    Single(&'a str),
}

/// Channel factory, which can be used to setup a new channel
///
/// This builder ease the configuration of the channel. You can chain up the configuration methods,
/// and call `build` to consume it, which will return a future that resolves to a `Channel`.
#[derive(Debug)]
pub struct ChannelBuilder<'a> {
    mode: ConnectMode<'a>,
    handle: Handle,
    protocol: Option<Protocol>,
    deadline: Option<Option<Duration>>,
    max_retry: Option<u32>,
    max_concurrency: Option<u32>,
}

impl<'a> ChannelBuilder<'a> {
    /// Connect to a server by IP address.
    ///
    /// This method will create a new channel builder.
    pub fn single_server(addr: &'a str, handle: Handle) -> Self {
        ChannelBuilder {
            mode: ConnectMode::Single(addr),
            handle: handle,
            protocol: None,
            deadline: None,
            max_retry: None,
            max_concurrency: None,
        }
    }

    /// [WIP] Choose a communication protocol.
    ///
    /// This RPC framework is intended to support multiple communication protocols
    /// (e.g. grpc and plain http). Currently, it is recommended to leave this to
    /// default value.
    ///
    /// Default to `brpc` protocol (`brpc` is a pure protobuf message protocol
    /// used in [brpc] framework).
    ///
    /// [brpc]: https://github.com/brpc/brpc
    ///
    pub fn protocol(mut self, protocol: Protocol) -> Self {
        self.protocol = Some(protocol);
        self
    }

    /// [WIP] Set request deadline.
    ///
    /// A request will be set to failed if it reaches its deadline.
    ///
    /// Default to `None`. which means we will wait until the reponse is returned or
    /// some error is raised.
    pub fn deadline(mut self, deadline: Option<Duration>) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Set concurrency limit.
    ///
    /// The number of unresolved requests will be confined below `max_concurrency`.
    /// When this limit is reached, new request will fail immidiately. Thus, it can
    /// be used to model backpressure.
    ///
    /// Default to `None`, no limit imposed.
    pub fn max_concurrency(mut self, max_concurrency: u32) -> Self {
        self.max_concurrency = Some(max_concurrency);
        self
    }

    /// Consume the builder and begin to prepare connection.
    ///
    /// This method returns a future that will resolve to a `Channel`.
    ///
    /// # Errors
    /// The future will yield a `ChannelBuildError` if any error occurs when seting
    /// up the connection.
    pub fn build(self) -> ChannelBuildFuture {
        // TODO: use Default trait
        let protocol = self.protocol.unwrap_or(Protocol::Brpc);
        // TODO: add timeout and retry
        let _deadline = self.deadline.unwrap_or(None);
        let max_concurrency = self.max_concurrency.unwrap_or(1_000_000);
        let handle = self.handle;

        let (tx, rx) = mpsc::unbounded();
        let channel = Channel::new(tx, max_concurrency);

        match self.mode {
            ConnectMode::Single(addr) => {
                let parse = addr.parse::<SocketAddr>()
                    .map_err(|e| ChannelBuildError::AddrParseError(e))
                    .into_future();
                let fut = parse.and_then(move |addr| {
                    let proto = MetaClientProtocol::new(&protocol, handle.clone(), addr.clone());
                    TcpClient::new(proto)
                        .connect(&addr, &handle)
                        .map_err(|_| ChannelBuildError::ConnectError)
                        .map(move |service| {
                            let end_port = ServerEndPort::new(service);
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

/// A future used internally by the framework. It will resolve to a serialized response.
#[derive(Debug)]
pub struct ChannelFuture {
    rx: Option<OneShotReceiver>,
    counter: Arc<AtomicUsize>,
}

impl ChannelFuture {
    /// Create a new future, used internally
    pub fn new(rx: Option<OneShotReceiver>, counter: Arc<AtomicUsize>) -> Self {
        ChannelFuture { rx, counter }
    }
}

impl Future for ChannelFuture {
    type Item = (ResponsePackage, FeedbackHandle);

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

/// Communication channel between servers
///
/// The `Channel` implements `Clone`, `Send`, and `Sync`. Once a channel is create from
/// a `ChannelBuilder`, it can be cloned and used in multiple threads.
#[derive(Clone, Debug)]
pub struct Channel {
    sender: ChannelSender,
    counter: Arc<AtomicUsize>,
    max_concurrency: usize,
}

impl Channel {
    /// Create a new channel.
    ///
    /// This method is used by `ChannelBuilder`.
    pub fn new(sender: ChannelSender, max_concurrency: u32) -> Self {
        Channel {
            sender,
            counter: Arc::new(AtomicUsize::new(0)),
            max_concurrency: max_concurrency as usize,
        }
    }

    /// Issue a request.
    ///
    /// This method deals with serialized, untyped message. It is meaned to be used
    /// internally by the framework. More ergonomic interfaces are provided by the 
    /// auto-generated stubs.
    pub fn call(&self, req: RequestPackage) -> ChannelFuture {
        let (tx, rx) = oneshot::channel();
        let rx = if self.counter.load(Ordering::SeqCst) < self.max_concurrency {
            self.counter.fetch_add(1, Ordering::SeqCst);
            self.sender
                .unbounded_send((tx, req))
                .expect("The receiving end is dropped");
            Some(rx)
        } else {
            None
        };

        ChannelFuture::new(rx, self.counter.clone())
    }

    // TODO: deprecate this
    /// Check if the channel is currently congested (i.e. concurrency limit is reached). 
    pub fn congested(&self) -> bool {
        let current = self.counter.load(Ordering::Relaxed);
        current >= self.max_concurrency
    }
}


/// [WIP] Feedback handle to load balancers
#[derive(Debug)]
pub struct FeedbackHandle {
    id: ServerId,
    sender: FeedbackSender,
}

impl FeedbackHandle {
    /// Create a new handle.
    pub fn new(id: ServerId, sender: FeedbackSender) -> Self {
        FeedbackHandle { id, sender }
    }

    /// Get server ID.
    pub fn server_id(&self) -> ServerId {
        self.id
    }

    /// Send feedback massage.
    pub fn call(self, info: CallInfo) {
        self.sender
            .send((self.id, info))
            .expect("The receiving end of the feedback channel is dropped");
    }
}
