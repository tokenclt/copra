pub use protobuf::ProtobufError;

use bytes::Bytes;
use bytes::buf::{FromBuf};
use protobuf::{parse_from_carllerche_bytes, Message, MessageStatic};
use std::marker::PhantomData;

pub trait MethodCodec {
    type Request;
    type Response;
    type Error;

    fn decode(&self, buf: Bytes) -> Result<Self::Request, Self::Error>;

    fn encode(&self, msg: Self::Response) -> Result<Bytes, Self::Error>;
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
    T: Message + MessageStatic,
    U: Message,
{
    type Request = T;
    type Response = U;
    type Error = ProtobufError;

    fn decode(&self, buf: Bytes) -> Result<Self::Request, Self::Error> {
        parse_from_carllerche_bytes(&buf)
    }

    fn encode(&self, msg: Self::Response) -> Result<Bytes, Self::Error> {
        let buf = msg.write_to_bytes()?;
        Ok(Bytes::from_buf(buf))
    }
}
