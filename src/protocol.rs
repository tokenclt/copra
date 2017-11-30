use bytes::BytesMut;
use futures::{future, Future};
use std::io;
use smallvec::SmallVec;
use tokio_io::codec::{Decoder, Encoder};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::multiplex::RequestId;

// Abstract over every protocols
pub struct Meta;

pub enum ProtocolError {
    TryOthers,
    NeedMoreBytes,
    AbsolutelyWrong,
}

trait RpcProtocol {
    fn try_parse(&self, buf: &mut BytesMut) -> Result<Option<Meta>, ProtocolError>;
}

pub struct BrpcProtocol;

impl RpcProtocol for BrpcProtocol {
    fn try_parse(&self, buf: &mut BytesMut) -> Result<Option<Meta>, ProtocolError> {
        unimplemented!()
    }
}

pub struct HttpProtocol;

impl RpcProtocol for HttpProtocol {
    fn try_parse(&self, buf: &mut BytesMut) -> Result<Option<Meta>, ProtocolError> {
        unimplemented!()
    }
}

pub struct ProtoAdapter {
    schemes: SmallVec<[Box<RpcProtocol>; 4]>,
    cached_scheme: Option<usize>,
}

impl ProtoAdapter {
    pub fn new() -> Self {
        ProtoAdapter {
            schemes: SmallVec::new(),
            cached_scheme: None,
        }
    }
}

impl Decoder for ProtoAdapter {
    type Item = (RequestId, Meta);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        unimplemented!()
    }
}

impl Encoder for ProtoAdapter {
    type Item = (RequestId, BytesMut);
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        unimplemented!()
    }
}


