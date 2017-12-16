use bytes::{BigEndian, Buf, BufMut, Bytes, BytesMut};
use futures::{future, Future};
use std::io;
use smallvec::SmallVec;
use tokio_io::codec::{Decoder, Encoder};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::multiplex::{ClientProto, RequestId, ServerProto};
use protobuf::{parse_from_carllerche_bytes, Message};

use message::{RpcMeta, RpcRequestMeta, RpcResponseMeta};

static HEADER: &[u8] = b"PRPC";

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

#[derive(Clone)]
enum BrpcParseState {
    ReadingHeader,
    ReadingLength,
    ReadingContent(u32, u32),
}

pub trait RpcProtocol: Sync + Send {
    fn try_parse(
        &mut self,
        buf: &mut BytesMut,
    ) -> Result<(RequestId, (RpcMeta, Bytes)), ProtocolError>;

    fn box_clone(&self) -> Box<RpcProtocol>;

    fn write_package(&self, meta: (RpcMeta, Bytes), buf: &mut BytesMut) -> io::Result<()>;
}

#[derive(Clone)]
pub struct BrpcProtocol {
    state: BrpcParseState,
}

impl BrpcProtocol {
    pub fn new() -> Self {
        BrpcProtocol {
            state: BrpcParseState::ReadingHeader,
        }
    }
}

impl RpcProtocol for BrpcProtocol {
    fn try_parse(
        &mut self,
        buf: &mut BytesMut,
    ) -> Result<(RequestId, (RpcMeta, Bytes)), ProtocolError> {
        loop {
            match self.state {
                BrpcParseState::ReadingHeader => {
                    if buf.len() < 4 {
                        return Err(ProtocolError::NeedMoreBytes);
                    }
                    {
                        let header = &buf[0..4];
                        if header != HEADER {
                            return Err(ProtocolError::TryOthers);
                        }
                    }
                    buf.split_to(4);
                    self.state = BrpcParseState::ReadingLength;
                }
                BrpcParseState::ReadingLength => {
                    if buf.len() < 8 {
                        return Err(ProtocolError::NeedMoreBytes);
                    }
                    let pkg_len = io::Cursor::new(buf.split_to(4)).get_u32::<BigEndian>();
                    let meta_len = io::Cursor::new(buf.split_to(4)).get_u32::<BigEndian>();
                    self.state = BrpcParseState::ReadingContent(pkg_len, meta_len);
                }
                BrpcParseState::ReadingContent(pkg_len, meta_len) => {
                    if buf.len() < pkg_len as usize {
                        return Err(ProtocolError::NeedMoreBytes);
                    }
                    let meta = parse_from_carllerche_bytes::<RpcMeta>(&buf.split_to(
                        meta_len as usize,
                    ).freeze())
                        .map_err(|_| ProtocolError::AbsolutelyWrong)?;
                    let body = buf.split_to((pkg_len - meta_len) as usize).freeze();
                    self.state = BrpcParseState::ReadingHeader;
                    return Ok((meta.get_correlation_id(), (meta, body)));
                }
            }
        }
    }

    fn box_clone(&self) -> Box<RpcProtocol> {
        Box::new(self.clone())
    }

    fn write_package(&self, meta: (RpcMeta, Bytes), buf: &mut BytesMut) -> io::Result<()> {
        let (meta, body) = meta;
        let meta_len = meta.compute_size();
        let body_len = body.len() as u32;

        let pkg_len = 12 + meta_len + body_len;
        let free_len = buf.remaining_mut() as u32;
        if free_len < pkg_len {
            buf.reserve((pkg_len - free_len) as usize);
        }

        buf.put_slice(HEADER);
        buf.put_u32::<BigEndian>(meta_len + body_len as u32);
        buf.put_u32::<BigEndian>(meta_len);
        buf.put(meta.write_to_bytes()?);
        buf.put(body);

        Ok(())
    }
}

#[derive(Clone)]
pub struct HttpProtocol;

impl RpcProtocol for HttpProtocol {
    fn try_parse(
        &mut self,
        buf: &mut BytesMut,
    ) -> Result<(RequestId, (RpcMeta, Bytes)), ProtocolError> {
        unimplemented!()
    }

    fn box_clone(&self) -> Box<RpcProtocol> {
        Box::new(self.clone())
    }

    fn write_package(&self, meta: (RpcMeta, Bytes), buf: &mut BytesMut) -> io::Result<()> {
        unimplemented!()
    }
}

pub struct ProtoCodec {
    schemes: SmallVec<[Box<RpcProtocol>; 4]>,
    cached_scheme: usize,
    tried_num: i32,
}

impl ProtoCodec {
    pub fn new(protos: &[Box<RpcProtocol>]) -> Self {
        let schemes: SmallVec<[Box<RpcProtocol>; 4]> =
            protos.iter().map(|proto| proto.box_clone()).collect();
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
                Ok((id, (mut meta, body))) => {
                    self.tried_num = 0;
                    if !meta.has_request() {
                        warn!("Request package do not have request field");
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "Request package do not have request field",
                        ));
                    }
                    return Ok(Some((id, (meta.take_request(), body))));
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
        let (id, (resp_meta, body)) = msg;
        let mut meta = RpcMeta::new();
        meta.set_response(resp_meta);
        meta.set_correlation_id(id);
        scheme.write_package((meta, body), buf)
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
        match self.scheme.try_parse(buf) {
            Ok((id, (mut meta, body))) => {
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
    type Item = (RequestId, RequestPackage);
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let (id, (request_meta, body)) = msg;
        let mut meta = RpcMeta::new();
        meta.set_request(request_meta);
        meta.set_correlation_id(id);

        self.scheme.write_package((meta, body), buf)
    }
}
