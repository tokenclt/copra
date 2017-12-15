use bytes::Bytes;
use protobuf::Message;
use std::marker::PhantomData;

pub trait MethodCodec {
    type Request;
    type Response;
    type Error;

    fn decode(&self, buf: Bytes) -> Result<Self::Request, Self::Error>;

    fn encode(&self, msg: Self::Response) -> Result<Bytes, Self::Error>;
}

#[derive(Clone, Debug)]
pub enum ProtobufCodecError {
    UnknownError,
}

#[derive(Clone)]
pub struct ProtobufCodec<T, U> {
    phantom: PhantomData<(T, U)>,
}

impl<T, U> ProtobufCodec<T, U> {
    pub fn new() -> Self {
        ProtobufCodec {
            phantom: PhantomData,
        }
    }
}

impl<T, U> MethodCodec for ProtobufCodec<T, U>
where
    T: Message,
    U: Message,
{
    type Request = T;
    type Response = U;
    type Error = ProtobufCodecError;

    fn decode(&self, buf: Bytes) -> Result<Self::Request, Self::Error> {
        unimplemented!()
    }

    fn encode(&self, msg: Self::Response) -> Result<Bytes, Self::Error> {
        unimplemented!()
    }
}
