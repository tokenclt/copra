//! Codecs for marshalling and unmarshalling messages

pub use protobuf::ProtobufError;

use bytes::Bytes;
use bytes::buf::FromBuf;
use protobuf::{parse_from_carllerche_bytes, Message, MessageStatic};
use std::marker::PhantomData;

/// Decode/encode messages from raw bytes
pub trait MethodCodec {
    /// Request message decoded from raw bytes
    type Request;
    /// Response message for encoding to raw bytes
    type Response;
    /// Error during decoding or encoding
    type Error;

    /// Decode message from bytes.
    fn decode(&self, buf: Bytes) -> Result<Self::Request, Self::Error>;

    /// Encode message to bytes.
    fn encode(&self, msg: Self::Response) -> Result<Bytes, Self::Error>;
}

/// Codec for protobuf messages
#[derive(Clone, Debug)]
pub struct ProtobufCodec<T, U> {
    phantom: PhantomData<(T, U)>,
}

impl<T, U> ProtobufCodec<T, U> {
    /// Create a new instance of the protobuf codec.
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
        parse_from_carllerche_bytes(&buf).map_err(|e| {
            error!("Failed to decode protobuf message from body");
            e
        })
    }

    fn encode(&self, msg: Self::Response) -> Result<Bytes, Self::Error> {
        let buf = msg.write_to_bytes().map_err(|e| {
            error!("Failed to encode protobuf message");
            e
        })?;
        Ok(Bytes::from_buf(buf))
    }
}
