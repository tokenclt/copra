//! Internal types helping to provide services

use bytes::Bytes;
use futures::{Future, IntoFuture};
use std::error::Error;
use std::fmt;
use std::io;
use tokio_service::NewService;

use controller::Controller;
use codec::{MethodCodec, ProtobufError};

pub use tokio_service::Service;

type Bundle = (Bytes, Controller);

/// A future that will resolve to a serialized message
pub type MethodFuture = Box<Future<Item = Bundle, Error = MethodError>>;

/// Type alias of `Service` trait object
///
/// This type is used internally by auto-generated stubs.
#[doc(hidden)]
pub type EncapService = Box<
    Service<Request = Bundle, Response = Bundle, Error = MethodError, Future = MethodFuture>
        + Send
        + Sync,
>;

/// Type alias of `NewService` trait object
///
/// This type is used internally by auto-generated stubs.
#[doc(hidden)]
pub type NewEncapService = Box<
    NewService<Request = Bundle, Response = Bundle, Error = MethodError, Instance = EncapService>
        + Send
        + Sync,
>;

/// A factory struct that can produce encapsulated service.
///
/// An encapsulated service consists of the method codec and the user-defined
/// processing logic.
pub struct NewEncapsulatedMethod<S: Send + Sync> {
    inner: Box<
        NewService<Request = Bundle, Response = Bundle, Error = MethodError, Instance = S>
            + Send
            + Sync,
    >,
}

impl<S> fmt::Debug for NewEncapsulatedMethod<S>
where
    S: Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NewEncapsulatedMethod")
    }
}

impl<S> NewEncapsulatedMethod<S>
where
    S: NewService<Request = Bundle, Response = Bundle, Error = MethodError, Instance = S>,
    S: Service<Request = Bundle, Response = Bundle, Error = MethodError, Future = MethodFuture>,
    S: 'static + Send + Sync,
{
    /// Create a new instance by boxing.
    pub fn new(method: S) -> Self {
        NewEncapsulatedMethod {
            inner: Box::new(method),
        }
    }
}

impl<S> NewService for NewEncapsulatedMethod<S>
where
    S: NewService<Request = Bundle, Response = Bundle, Error = MethodError, Instance = S>,
    S: Service<Request = Bundle, Response = Bundle, Error = MethodError, Future = MethodFuture>,
    S: 'static + Send + Sync,
{
    type Request = Bundle;
    type Response = Bundle;
    type Error = MethodError;
    type Instance = EncapService;

    fn new_service(&self) -> io::Result<Self::Instance> {
        self.inner
            .new_service()
            .map(|s| Box::new(s) as EncapService)
    }
}

// TODO: seperate this into server error and stub error
/// [WIP] Error return by service providers
#[derive(Clone, Debug)]
pub enum MethodError {
    /// [WIP] Other errors that might be worth discussion
    UnknownError,
    /// Failed to decode message
    CodecError,
}

impl fmt::Display for MethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MethodError::UnknownError => write!(f, "unknown error produced by server"),
            MethodError::CodecError => write!(f, "failed to decode message"),
        }
    }
}

impl Error for MethodError {
    fn description(&self) -> &str {
        match *self {
            MethodError::UnknownError => "unknown error",
            MethodError::CodecError => "codec error",
        }
    }
}

#[doc(hidden)]
impl From<ProtobufError> for MethodError {
    fn from(_: ProtobufError) -> Self {
        MethodError::CodecError
    }
}


/// A bunble of a codec and a user-defined service
/// 
/// This struct is used to provide a unified interface for the request dispatcher.
#[allow(missing_debug_implementations)]
pub struct EncapsulatedMethod<C, S> {
    codec: C,
    method: S,
}

impl<C, S> EncapsulatedMethod<C, S> where {
    /// Create a new bundle from a codec and a service
    pub fn new(codec: C, method: S) -> Self {
        EncapsulatedMethod {
            codec: codec,
            method: method,
        }
    }
}

impl<C, S> Service for EncapsulatedMethod<C, S>
where
    C: MethodCodec + Clone + 'static,
    S: Service<
        Request = (C::Request, Controller),
        Response = (C::Response, Controller),
        Error = MethodError,
    >,
    S: Clone + 'static,
    MethodError: From<C::Error>,
{
    type Request = Bundle;
    type Response = Bundle;
    type Error = MethodError;
    type Future = MethodFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        let codec = self.codec.clone();
        let method = self.method.clone();
        let (body, controller) = req;
        let fut = codec
            .decode(body)
            .map_err(|e| e.into())
            .into_future()
            .and_then(move |body| {
                method
                    .call((body, controller))
                    .map_err(|_| MethodError::UnknownError)
                    .and_then(move |(body, controller)| {
                        codec
                            .encode(body)
                            .map_err(|e| e.into())
                            .map(|body| (body, controller))
                    })
            });
        Box::new(fut)
    }
}

impl<C, S> NewService for EncapsulatedMethod<C, S>
where
    C: MethodCodec + Clone + 'static,
    MethodError: From<C::Error>,
    S: Service<
        Request = (C::Request, Controller),
        Response = (C::Response, Controller),
        Error = MethodError,
    >,
    S: Clone + 'static,
{
    type Request = Bundle;
    type Response = Bundle;
    type Error = MethodError;
    type Instance = Self;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(EncapsulatedMethod {
            codec: self.codec.clone(),
            method: self.method.clone(),
        })
    }
}
