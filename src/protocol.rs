use bytes::{Bytes, BytesMut};
use futures::{future, Future};
use std::io;
use smallvec::SmallVec;
use tokio_io::codec::{Decoder, Encoder};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::multiplex::{ClientProto, RequestId, ServerProto};

use message::{RpcMeta, RpcRequestMeta, RpcResponseMeta};

type RequestPackage = (RpcRequestMeta, Bytes);

type ResponsePackage = (RpcResponseMeta, Bytes);
// Abstract over every protocols

#[derive(Clone, Debug)]
pub enum Protocol {
    Brpc,
    Http,
}

pub enum ProtocolError {
    TryOthers,
    NeedMoreBytes,
    AbsolutelyWrong,
}

pub trait RpcProtocol: Sync + Send {
    fn try_parse(&self, buf: &mut BytesMut) -> Result<Option<RequestPackage>, ProtocolError>;

    fn box_clone(&self) -> Box<RpcProtocol>;
}

#[derive(Clone)]
pub struct BrpcProtocol;

impl RpcProtocol for BrpcProtocol {
    fn try_parse(&self, buf: &mut BytesMut) -> Result<Option<RequestPackage>, ProtocolError> {
        unimplemented!()
    }

    fn box_clone(&self) -> Box<RpcProtocol> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct HttpProtocol;

impl RpcProtocol for HttpProtocol {
    fn try_parse(&self, buf: &mut BytesMut) -> Result<Option<RequestPackage>, ProtocolError> {
        unimplemented!()
    }

    fn box_clone(&self) -> Box<RpcProtocol> {
        Box::new(self.clone())
    }
}

pub struct ProtoCodec {
    schemes: SmallVec<[Box<RpcProtocol>; 4]>,
    cached_scheme: Option<usize>,
}

impl ProtoCodec {
    pub fn new(protos: &[Box<RpcProtocol>]) -> Self {
        let schemes: SmallVec<[Box<RpcProtocol>; 4]> =
            protos.iter().map(|proto| proto.box_clone()).collect();
        ProtoCodec {
            schemes,
            cached_scheme: None,
        }
    }
}

impl Decoder for ProtoCodec {
    type Item = (RequestId, RequestPackage);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        unimplemented!()
    }
}

impl Encoder for ProtoCodec {
    type Item = (RequestId, ResponsePackage);
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        unimplemented!()
    }
}


pub struct ProtoCodecClient {
    scheme: Box<RpcProtocol>,
}

impl ProtoCodecClient {
    pub fn new(proto: Box<RpcProtocol>) -> Self {
        ProtoCodecClient { scheme: proto }
    }
}

impl Decoder for ProtoCodecClient {
    type Item = (RequestId, ResponsePackage);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        unimplemented!()
    }
}

impl Encoder for ProtoCodecClient {
    type Item = (RequestId, RequestPackage);
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        unimplemented!()
    }
}
