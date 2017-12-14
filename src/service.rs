use bytes::BytesMut;
use futures::Future;
use protobuf::Message;
use std::marker::PhantomData;
use std::error;
use std::io;
use tokio_service::{NewService, Service};

use protocol::Meta;
use codec::{MethodCodec, ProtobufCodecError};

type StdError = error::Error;

pub type MethodFuture<'a> = Box<Future<Item = Meta, Error = MethodError> + 'a>;

pub type EncapService<'a> = Box<
    Service<Request = Meta, Response = Meta, Error = MethodError, Future = MethodFuture<'a>> + 'a,
>;

pub type NewEncapService<'a> = Box<
    NewService<Request = Meta, Response = Meta, Error = MethodError, Instance = EncapService<'a>>
        + 'a,
>;

pub struct NewEncapsulatedMethod<'a, S: 'a> {
    inner: Box<NewService<Request = Meta, Response = Meta, Error = MethodError, Instance = S> + 'a>,
}

impl<'a, S> NewEncapsulatedMethod<'a, S>
where
    S: NewService<Request = Meta, Response = Meta, Error = MethodError, Instance = S>,
    S: Service<Request = Meta, Response = Meta, Error = MethodError, Future = MethodFuture<'a>>,
    S: 'a,
{
    pub fn new(method: S) -> Self {
        NewEncapsulatedMethod {
            inner: Box::new(method),
        }
    }
}

impl<'a, S> NewService for NewEncapsulatedMethod<'a, S>
where
    S: NewService<Request = Meta, Response = Meta, Error = MethodError, Instance = S>,
    S: Service<Request = Meta, Response = Meta, Error = MethodError, Future = MethodFuture<'a>>,
    S: 'a,
{
    type Request = Meta;
    type Response = Meta;
    type Error = MethodError;
    type Instance = EncapService<'a>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        self.inner
            .new_service()
            .map(|s| Box::new(s) as EncapService)
    }
}

#[derive(Clone)]
pub enum MethodError {
    UnknownError,
    CodecError(ProtobufCodecError),
}

impl From<ProtobufCodecError> for MethodError {
    fn from(e: ProtobufCodecError) -> Self {
        MethodError::CodecError(e)
    }
}

pub struct EncapsulatedMethod<'a, C, S: 'a> {
    codec: C,
    method: S,
    phantom: PhantomData<&'a ()>,
}

impl<'a, C, S: 'a> EncapsulatedMethod<'a, C, S> where {
    pub fn new(codec: C, method: S) -> Self {
        EncapsulatedMethod {
            codec,
            method,
            phantom: PhantomData,
        }
    }
}

impl<'a, C, S> Service for EncapsulatedMethod<'a, C, S>
where
    C: MethodCodec<Request = S::Request, Response = S::Response>,
    S: Service + 'a,
{
    type Request = Meta;
    type Response = Meta;
    type Error = MethodError;
    type Future = MethodFuture<'a>;

    fn call(&self, req: Self::Request) -> Self::Future {
        unimplemented!()
    }
}

impl<'a, C, S> NewService for EncapsulatedMethod<'a, C, S>
where
    C: MethodCodec<Request = S::Request, Response = S::Response> + Clone,
    S: Service + Clone + 'a,
{
    type Request = Meta;
    type Response = Meta;
    type Error = MethodError;
    type Instance = Self;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(EncapsulatedMethod::new(
            self.codec.clone(),
            self.method.clone(),
        ))
    }
}
