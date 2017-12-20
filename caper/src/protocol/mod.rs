use bytes::{Bytes, BytesMut};
use smallvec::SmallVec;
use std::io;
use tokio_io::codec::{Decoder, Encoder};
use tokio_proto::multiplex::RequestId;

use controller::Controller;
use message::{RpcMeta, RpcRequestMeta, RpcResponseMeta};
use message::{RequestPackage, ResponsePackage};

pub use self::brpc::BrpcProtocol;
pub use self::http::HttpProtocol;

pub mod brpc;
pub mod http;

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
    fn try_parse(
        &mut self,
        buf: &mut BytesMut,
    ) -> Result<(RequestId, (RpcMeta, Controller, Bytes)), ProtocolError>;

    fn new_boxed(&self) -> Box<RpcProtocol>;

    fn write_package(
        &self,
        meta: (RpcMeta, Controller, Bytes),
        buf: &mut BytesMut,
    ) -> io::Result<()>;
}

pub struct ProtoCodec {
    schemes: SmallVec<[Box<RpcProtocol>; 4]>,
    cached_scheme: usize,
    tried_num: i32,
}

impl ProtoCodec {
    pub fn new(protos: &[Box<RpcProtocol>]) -> Self {
        let schemes: SmallVec<[Box<RpcProtocol>; 4]> =
            protos.iter().map(|proto| proto.new_boxed()).collect();
        ProtoCodec {
            schemes,
            cached_scheme: 0,
            tried_num: 0,
        }
    }
}

impl Decoder for ProtoCodec {
    type Item = (RequestId, RequestPackage);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        loop {
            match self.schemes[self.cached_scheme].try_parse(buf) {
                Ok((id, (mut meta, controller, body))) => {
                    self.tried_num = 0;
                    // if !meta.has_request() {
                    //     warn!("Request package do not have request field");
                    //     return Err(io::Error::new(
                    //         io::ErrorKind::Other,
                    //         "Request package do not have request field",
                    //     ));
                    // }
                    return Ok(Some((id, (meta.take_request(), controller, body))));
                }
                Err(ProtocolError::NeedMoreBytes) => return Ok(None),
                Err(ProtocolError::TryOthers) => {
                    self.cached_scheme = (self.cached_scheme + 1) % self.schemes.len();
                    self.tried_num += 1;
                    if self.tried_num >= self.schemes.len() as i32 {
                        self.tried_num = 0;
                        warn!("No protocol recognize this package");
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "No protocol recognize this package",
                        ));
                    }
                }
                Err(ProtocolError::AbsolutelyWrong) => {
                    warn!("Invalid request package");
                    return Err(io::Error::new(io::ErrorKind::Other, "Invalid package"));
                }
            }
        }
    }
}

impl Encoder for ProtoCodec {
    type Item = (RequestId, ResponsePackage);
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let scheme = &self.schemes[self.cached_scheme];
        let (id, (resp_meta, controller, body)) = msg;
        let mut meta = RpcMeta::new();
        meta.set_response(resp_meta);
        meta.set_correlation_id(id);
        scheme.write_package((meta, controller, body), buf)
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
    type Item = (RequestId, (RpcResponseMeta, Bytes));
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.scheme.try_parse(buf) {
            Ok((id, (mut meta, _, body))) => {
                if !meta.has_response() {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Response package do not have response field",
                    ));
                }
                return Ok(Some((id, (meta.take_response(), body))));
            }
            Err(ProtocolError::NeedMoreBytes) => return Ok(None),
            Err(ProtocolError::TryOthers) | Err(ProtocolError::AbsolutelyWrong) => {
                error!("Decode response package failed, invalid package or wrong protocol");
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Invalid package or wrong protocol",
                ));
            }
        }
    }
}

impl Encoder for ProtoCodecClient {
    type Item = (RequestId, (RpcRequestMeta, Bytes));
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let (id, (request_meta, body)) = msg;
        let mut meta = RpcMeta::new();
        meta.set_request(request_meta);
        meta.set_correlation_id(id);

        self.scheme
            .write_package((meta, Controller::default(), body), buf)
    }
}
