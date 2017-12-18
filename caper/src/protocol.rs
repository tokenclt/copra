use bytes::{BigEndian, Buf, BufMut, Bytes, BytesMut};
use futures::{future, Future};
use std::io;
use std::str;
use std::collections::HashMap;
use smallvec::SmallVec;
use tokio_io::codec::{Decoder, Encoder};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::multiplex::{ClientProto, RequestId, ServerProto};
use httparse::Header as HttpHeader;
use httparse::Request as HttpRequest;
use httparse::Error as HttpParseError;
use httparse::Status;
use httparse;
use protobuf::{parse_from_carllerche_bytes, Message};

use controller::Controller;
use message::{RpcMeta, RpcRequestMeta, RpcResponseMeta};
use message::{RequestPackage, ResponsePackage};

static HEADER: &[u8] = b"PRPC";

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
    ) -> Result<(RequestId, (RpcMeta, Controller, Bytes)), ProtocolError>;

    fn box_clone(&self) -> Box<RpcProtocol>;

    fn write_package(
        &self,
        meta: (RpcMeta, Controller, Bytes),
        buf: &mut BytesMut,
    ) -> io::Result<()>;
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
    ) -> Result<(RequestId, (RpcMeta, Controller, Bytes)), ProtocolError> {
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
                    return Ok((
                        meta.get_correlation_id(),
                        (meta, Controller::default(), body),
                    ));
                }
            }
        }
    }

    fn box_clone(&self) -> Box<RpcProtocol> {
        Box::new(self.clone())
    }

    fn write_package(
        &self,
        meta: (RpcMeta, Controller, Bytes),
        buf: &mut BytesMut,
    ) -> io::Result<()> {
        let (meta, _, body) = meta;
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
enum HttpParseState {
    ReadingHeader,
    /// (header length, content length, controller)
    ReadingContent(usize, usize, RpcMeta, Controller),
}

#[derive(Clone)]
pub struct HttpProtocol {
    state: HttpParseState,
}

impl HttpProtocol {
    pub fn new() -> Self {
        HttpProtocol {
            state: HttpParseState::ReadingHeader,
        }
    }

    fn parse_name(&self, path: &str) -> Result<(String, String), ProtocolError> {
        unimplemented!()
    }
}

impl RpcProtocol for HttpProtocol {
    fn try_parse(
        &mut self,
        buf: &mut BytesMut,
    ) -> Result<(RequestId, (RpcMeta, Controller, Bytes)), ProtocolError> {
        loop {
            match self.state {
                HttpParseState::ReadingHeader => {
                    let mut headers = [httparse::EMPTY_HEADER; 10];
                    let mut req = HttpRequest::new(&mut headers);
                    match req.parse(buf) {
                        Ok(Status::Complete(header_len)) => {
                            let mut controller = Controller::default();
                            let mut header_map = HashMap::new();
                            let mut request_meta = RpcRequestMeta::new();
                            let mut meta = RpcMeta::new();

                            for header in req.headers {
                                let val = str::from_utf8(header.value)
                                    .map_err(|_| ProtocolError::AbsolutelyWrong)?;
                                header_map.insert(header.name.to_string(), val.to_string());
                            }

                            let path = req.path.ok_or(ProtocolError::AbsolutelyWrong)?;
                            let id = header_map
                                .get("Correlation-Id")
                                .ok_or(ProtocolError::AbsolutelyWrong)
                                .and_then(|s| {
                                    s.parse().map_err(|_| ProtocolError::AbsolutelyWrong)
                                })?;
                            let content_len = header_map
                                .get("Content-Length")
                                .ok_or(ProtocolError::AbsolutelyWrong)
                                .and_then(|s| {
                                    s.parse().map_err(|_| ProtocolError::AbsolutelyWrong)
                                })?;
                            controller.http_url = Some(path.to_string());
                            controller.headers = header_map;

                            let (service, method) = self.parse_name(path)?;
                            request_meta.set_service_name(service);
                            request_meta.set_method_name(method);

                            meta.set_request(request_meta);
                            meta.set_correlation_id(id);

                            self.state = HttpParseState::ReadingContent(
                                header_len,
                                content_len,
                                meta,
                                controller,
                            );
                        }
                        Ok(Status::Partial) => return Err(ProtocolError::NeedMoreBytes),
                        Err(_) => return Err(ProtocolError::AbsolutelyWrong),
                    }
                }
                HttpParseState::ReadingContent(header_len, content_len, ..) => {
                    if buf.len() < (header_len + content_len) {
                        return Err(ProtocolError::NeedMoreBytes);
                    }

                    let state = ::std::mem::replace(&mut self.state, HttpParseState::ReadingHeader);

                    buf.split_to(header_len);

                    if let HttpParseState::ReadingContent(.., meta, mut controller) = state {
                        let body = buf.split_to(content_len as usize).freeze();
                        controller.request_body = Vec::from(body.as_ref());
                        self.state = HttpParseState::ReadingHeader;
                        return Ok((meta.get_correlation_id(), (meta, controller, Bytes::new())));
                    } else {
                        unreachable!();
                    }
                }
            }
        }
    }

    fn box_clone(&self) -> Box<RpcProtocol> {
        Box::new(self.clone())
    }

    fn write_package(
        &self,
        meta: (RpcMeta, Controller, Bytes),
        buf: &mut BytesMut,
    ) -> io::Result<()> {
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
                Ok((id, (mut meta, controller, body))) => {
                    self.tried_num = 0;
                    if !meta.has_request() {
                        warn!("Request package do not have request field");
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "Request package do not have request field",
                        ));
                    }
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
