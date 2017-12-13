use bytes::{Bytes, BytesMut};
use futures::{future, Future};
use std::io;
use smallvec::SmallVec;
use tokio_io::codec::{Decoder, Encoder};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::multiplex::{RequestId, ServerProto, ClientProto};

// Abstract over every protocols
#[derive(Debug)]
pub struct Meta {
    pub service_name: String,
    pub method_name: String,
    pub body: Bytes,
}

type MultiplexedFrame = (RequestId, Meta);

pub enum ProtocolError {
    TryOthers,
    NeedMoreBytes,
    AbsolutelyWrong,
}

pub trait RpcProtocol {
    fn try_parse(&self, buf: &mut BytesMut) -> Result<Option<Meta>, ProtocolError>;

    fn box_clone(&self) -> Box<RpcProtocol>;
}

#[derive(Clone)]
pub struct BrpcProtocol;

impl RpcProtocol for BrpcProtocol {
    fn try_parse(&self, buf: &mut BytesMut) -> Result<Option<Meta>, ProtocolError> {
        unimplemented!()
    }

    fn box_clone(&self) -> Box<RpcProtocol> {
        Box::new(self.clone())
    }

}

#[derive(Clone)]
pub struct HttpProtocol;

impl RpcProtocol for HttpProtocol {
    fn try_parse(&self, buf: &mut BytesMut) -> Result<Option<Meta>, ProtocolError> {
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
    pub fn new() -> Self {
        ProtoCodec {
            schemes: SmallVec::new(),
            cached_scheme: None,
        }
    }
}

impl Decoder for ProtoCodec {
    type Item = (RequestId, Meta);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        unimplemented!()
    }
}

impl Encoder for ProtoCodec {
    type Item = (RequestId, Meta);
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

pub struct MetaServerProtocol;

impl<T> ServerProto<T> for MetaServerProtocol
where
    T: AsyncRead + AsyncWrite + 'static,
{
    type Request = Meta;
    type Response = Meta;
    type Transport = Framed<T, ProtoCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(ProtoCodec::new()))
    }
}

