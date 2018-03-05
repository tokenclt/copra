//! Types that help to generate RPC stubs

use futures::{Async, Future, Poll};

use codec::MethodCodec;
use channel::{Channel, ChannelFuture, ChannelError};
use load_balancer::CallInfo;
use message::{RpcRequestMeta};

/// The error type for an RPC request
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RpcError {
    kind: RpcErrorKind,
    text: String,
}

impl RpcError {
    /// Create a new error with error kind and error description
    pub fn new(kind: RpcErrorKind, text: String) -> Self {
        RpcError { kind, text }
    }

    /// Get kind
    pub fn kind(&self) -> RpcErrorKind {
        self.kind
    }

    /// Get error text
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl From<ChannelError> for RpcError {
    fn from(_e: ChannelError) -> Self {
        unimplemented!()
    }
}

/// The error kind for an RPC request
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RpcErrorKind {
    /// Server complains that the request message is invalid
    ///
    /// This error is returned when the server fails to deserialize
    /// the request (e.g. uninitialized field exists in protobuf message),
    /// or the request message violates some predefined contracts.
    InvalidRequest,

    /// The server failed to carry on some internal logics
    InteralServerError,

    /// Failed to connect the server at the provided address
    ServerNotFound,

    /// The requested service is not found
    ServiceNotFound,

    /// The requested method is not found with the service namespace
    MethodNotFound,

    /// Failed to serialize the request
    RequestEncodeError,

    /// Failed to deserialize the response
    ResponseDecodeError,

    /// The server is overcrowded, and the request is refused
    ServerOvercrowded,

    /// The deadline is reached
    TimeOut,

    /// The error stems from TCP connection
    ///
    /// This error is returned when the connection is broken or shutdown by
    /// the server due to unrecoverable parse errors.
    BrokenConnection,

    /// The underlying channel reached its concurrency limit
    ChannelOvercrowded,
}

impl RpcErrorKind {
    /// Create error kind from error code
    pub fn from_error_code(_code: i32) -> Self {
        unimplemented!()
    }
}

/// Bind a stub to a [`Channel`]
///
/// [`Channel`]: ../channel/struct.Channel.html
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct RpcWrapper<'a, C: Clone> {
    codec: C,
    channel: &'a Channel,
}

impl<'a, C: Clone> RpcWrapper<'a, C> {
    /// Create a binding from a codec and a reference to channel.
    pub fn new(codec: C, channel: &'a Channel) -> Self {
        RpcWrapper { codec, channel }
    }
}

impl<'a, C> RpcWrapper<'a, C>
where
    C: MethodCodec + Clone,
{
    /// Issue a request and obtain a future.
    pub fn call(&'a self, bundle: (C::Response, String, String)) -> StubFuture<C> {
        let (req, service_name, method_name) = bundle;
        let channel_fut = match self.codec.encode(req) {
            Ok(body) => {
                let mut meta = RpcRequestMeta::new();
                meta.set_service_name(service_name);
                meta.set_method_name(method_name);
                Some(self.channel.call((meta, body)))
            }
            Err(_) => None,
        };

        StubFuture::new(channel_fut, self.codec.clone())
    }
}

/// A future that will resolve to a pair of response and RPC info
#[derive(Debug)]
pub struct StubFuture<C> {
    start_usec: u64,
    inner: Option<ChannelFuture>,
    codec: C,
}

impl<C> StubFuture<C> {
    /// Create a new future.
    pub fn new(inner: Option<ChannelFuture>, codec: C) -> Self {
        StubFuture {
            start_usec: 0,
            inner,
            codec,
        }
    }
}

impl<C> Future for StubFuture<C>
where
    C: MethodCodec,
{
    type Item = (C::Request, RpcInfo);

    type Error = RpcError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if let Some(ref mut channel) = self.inner {
            match channel.poll() {
                Ok(Async::Ready((resp, fb_handle))) => {
                    let (meta, body) = resp;

                    if meta.get_error_code() == 0 {
                        let resp = self.codec.decode(body).map_err(|_| {
                            RpcError::new(
                                RpcErrorKind::ResponseDecodeError,
                                "response decode error".to_string(),
                            )
                        })?;
                        let info = RpcInfo;
                        let fb = CallInfo::new(self.start_usec, None);
                        fb_handle.call(fb);

                        Ok(Async::Ready((resp, info)))
                    } else {
                        let kind = RpcErrorKind::from_error_code(meta.get_error_code());
                        let err = RpcError::new(kind, meta.error_text);
                        let fb = CallInfo::new(self.start_usec, Some(kind));
                        fb_handle.call(fb);

                        Err(err)
                    }
                }
                Ok(Async::NotReady) => Ok(Async::NotReady),
                // TODO: Add error convertion
                Err(e) => Err(RpcError::from(e)),
            }
        } else {
            Err(RpcError::new(
                RpcErrorKind::RequestEncodeError,
                "request encode error".to_string(),
            ))
        }
    }
}

/// [WIP] Information about how the RPC request has been processed
#[derive(Clone, Debug, PartialEq)]
pub struct RpcInfo;
