use bytes::BytesMut;
use futures::Future;
use protobuf::Message;
use std::marker::PhantomData;
use std::error;
use tokio_service::Service;

use protocol::Meta;
use codec::{MethodCodec, ProtobufCodecError};

type StdError = error::Error;

pub type MethodFuture = Box<Future<Item = BytesMut, Error = MethodError>>;

pub enum MethodError {
    UnknownError,
    CodecError(ProtobufCodecError),
}

impl From<ProtobufCodecError> for MethodError {
    fn from(e: ProtobufCodecError) -> Self {
        MethodError::CodecError(e)
    }
}

pub struct EncapsulatedMethod<C, S, T, U, E> {
    codec: C,
    method: S,
    phantom: PhantomData<(T, U, E)>,
}

impl<C, S, T, U, E> EncapsulatedMethod<C, S, T, U, E>
where
    C: MethodCodec<Request = T, Response = U, Error = E>,
    S: Service<Request = T, Response = U, Error = MethodError>,
{
    pub fn new(codec: C, method: S) -> Self {
        EncapsulatedMethod {
            codec,
            method,
            phantom: PhantomData,
        }
    }
}

impl<C, S, T, U, E> Service for EncapsulatedMethod<C, S, T, U, E>
where
    C: MethodCodec<Request = T, Response = U, Error = E>,
    S: Service<Request = T, Response = U, Error = MethodError>,
{
    type Request = Meta;
    type Response = BytesMut;
    type Error = MethodError;
    type Future = MethodFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        unimplemented!()
    }
}


