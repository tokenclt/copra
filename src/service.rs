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

pub type MethodFuture = Box<Future<Item = BytesMut, Error = MethodError>>;

pub type EncapService<'a> = Box<
    Service<Request = Meta, Response = BytesMut, Error = MethodError, Future = MethodFuture> + 'a,
>;

pub type NewEncapService<'a> = Box<
    NewService<
        Request = Meta,
        Response = BytesMut,
        Error = MethodError,
        Instance = EncapService<'a>,
    >,
>;

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

pub struct EncapsulatedMethod<'a, C, S, T, U, E>
where
    C: Clone,
    S: Clone + 'a,
{
    codec: C,
    method: S,
    phantom: PhantomData<(&'a (), T, U, E)>,
}

impl<'a, C, S, T, U, E> EncapsulatedMethod<'a, C, S, T, U, E>
where
    C: MethodCodec<Request = T, Response = U, Error = E> + Clone,
    S: Service<Request = T, Response = U, Error = MethodError> + Clone + 'a,
{
    pub fn new(codec: C, method: S) -> Self {
        EncapsulatedMethod {
            codec,
            method,
            phantom: PhantomData,
        }
    }
}

impl<'a, C, S, T, U, E> Service for EncapsulatedMethod<'a, C, S, T, U, E>
where
    C: MethodCodec<Request = T, Response = U, Error = E> + Clone,
    S: Service<Request = T, Response = U, Error = MethodError> + Clone + 'a,
{
    type Request = Meta;
    type Response = BytesMut;
    type Error = MethodError;
    type Future = MethodFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        unimplemented!()
    }
}

impl<'a, C, S, T, U, E> NewService for EncapsulatedMethod<'a, C, S, T, U, E>
where
    C: MethodCodec<Request = T, Response = U, Error = E> + Clone + 'static,
    S: Service<Request = T, Response = U, Error = MethodError> + Clone + 'a,
    T: 'static,
    U: 'static,
    E: 'static,
{
    type Request = Meta;
    type Response = BytesMut;
    type Error = MethodError;
    type Instance = EncapService<'a>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(Box::new(EncapsulatedMethod::new(
            self.codec.clone(),
            self.method.clone(),
        )))
    }
}
