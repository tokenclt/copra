use bytes::{Bytes, BytesMut};
use futures::{Future, IntoFuture};
use protobuf::{parse_from_carllerche_bytes, Message, MessageStatic};
use std::marker::PhantomData;
use std::error;
use std::io;
use tokio_service::{NewService};
use std::sync::Arc;

use codec::{MethodCodec, ProtobufError};
use message::{RpcMeta, RpcRequestMeta, RpcResponseMeta};

pub use tokio_service::Service;

type StdError = error::Error;

type Body = Bytes;

pub type MethodFuture = Box<Future<Item = Body, Error = MethodError>>;

pub type EncapService = Box<
    Service<Request = Body, Response = Body, Error = MethodError, Future = MethodFuture>
        + Send
        + Sync,
>;

pub type NewEncapService = Box<
    NewService<Request = Body, Response = Body, Error = MethodError, Instance = EncapService>
        + Send
        + Sync,
>;

pub struct NewEncapsulatedMethod<S: Send + Sync> {
    inner: Box<
        NewService<Request = Body, Response = Body, Error = MethodError, Instance = S>
            + Send
            + Sync,
    >,
}

impl<S> NewEncapsulatedMethod<S>
where
    S: NewService<Request = Body, Response = Body, Error = MethodError, Instance = S>,
    S: Service<Request = Body, Response = Body, Error = MethodError, Future = MethodFuture>,
    S: 'static + Send + Sync,
{
    pub fn new(method: S) -> Self {
        NewEncapsulatedMethod {
            inner: Box::new(method),
        }
    }
}

impl<S> NewService for NewEncapsulatedMethod<S>
where
    S: NewService<Request = Body, Response = Body, Error = MethodError, Instance = S>,
    S: Service<Request = Body, Response = Body, Error = MethodError, Future = MethodFuture>,
    S: 'static + Send + Sync,
{
    type Request = Body;
    type Response = Body;
    type Error = MethodError;
    type Instance = EncapService;

    fn new_service(&self) -> io::Result<Self::Instance> {
        self.inner
            .new_service()
            .map(|s| Box::new(s) as EncapService)
    }
}

#[derive(Debug)]
pub enum MethodError {
    UnknownError,
    CodecError(ProtobufError),
}

impl From<ProtobufError> for MethodError {
    fn from(e: ProtobufError) -> Self {
        MethodError::CodecError(e)
    }
}

pub struct EncapsulatedMethod<C, S> {
    codec: C,
    method: S,
}

impl<C, S> EncapsulatedMethod<C, S> where {
    pub fn new(codec: C, method: S) -> Self {
        EncapsulatedMethod {
            codec: codec,
            method: method,
        }
    }
}

impl<C, S> Service for EncapsulatedMethod<C, S>
where
    C: MethodCodec<Request = S::Request, Response = S::Response> + Clone + 'static,
    MethodError: From<C::Error>,
    S: Service + Clone + 'static,
{
    type Request = Body;
    type Response = Body;
    type Error = MethodError;
    type Future = MethodFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        let codec = self.codec.clone();
        let method = self.method.clone();

        let fut = codec
            .decode(req)
            .map_err(|e| e.into())
            .into_future()
            .and_then(move |req| {
                method
                    .call(req)
                    .map_err(|_| MethodError::UnknownError)
                    .and_then(move |resp| codec.encode(resp).map_err(|e| e.into()))
            });
        Box::new(fut)
    }
}

impl<C, S> NewService for EncapsulatedMethod<C, S>
where
    C: MethodCodec<Request = S::Request, Response = S::Response> + Clone + 'static,
    MethodError: From<C::Error>,
    S: Service + Clone + 'static,
{
    type Request = Body;
    type Response = Body;
    type Error = MethodError;
    type Instance = Self;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(EncapsulatedMethod {
            codec: self.codec.clone(),
            method: self.method.clone(),
        })
    }
}
