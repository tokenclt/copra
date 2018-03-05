//! Message protocols

use bytes::{Bytes, BytesMut};
use smallvec::SmallVec;
use std::fmt;
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

// TODO: depracate
/// Protocol selection enum
#[derive(Clone, Debug)]
pub enum Protocol {
    /// brpc protocol
    Brpc,
    /// plain http 1.X protocol
    Http,
}

/// Protocol resolution error at server side
#[derive(Clone, Debug, PartialEq)]
pub enum ProtocolError {
    /// The byte stream does not match the format of this protocol,
    /// try next protocol
    TryOthers,
    /// Can not decide if the byte stream matches this protocol, need more data
    NeedMoreBytes,
    /// The byte stream has partially matched this protocol, but now there is
    /// a decoding error
    AbsolutelyWrong,
}

/// A protocl that can decode and encode RPC messages
pub trait RpcProtocol: Sync + Send {
    /// Test if the byte stream matches this protocol.
    fn try_parse(
        &mut self,
        buf: &mut BytesMut,
    ) -> Result<(RequestId, (RpcMeta, Controller, Bytes)), ProtocolError>;

    /// Clone and wrap into a box.
    fn new_boxed(&self) -> Box<RpcProtocol>;

    /// encode message to bytes and add them to the buffer.
    fn write_package(
        &self,
        meta: (RpcMeta, Controller, Bytes),
        buf: &mut BytesMut,
    ) -> io::Result<()>;

    /// Protocol name.
    fn name(&self) -> &'static str;
}

impl fmt::Debug for RpcProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Server side codec that can deduce protocol from byte stream
///
/// Server can provide services to clients that use different protocols.
/// When a new connection is established, the server try each protocol until
/// it succeeds in decoding the request. Since `copra` use keep-alive connections
/// to exchange messages, this match is cached so that the protocol resolution
/// overhead is only incurred when receiving the first request.
#[derive(Debug)]
pub struct ProtoCodec {
    schemes: SmallVec<[Box<RpcProtocol>; 4]>,
    cached_scheme: usize,
    tried_num: i32,
}

impl ProtoCodec {
    /// Create a new codec that support multiple protocols.
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

// TODO: Should we handle the error case that the protocol header (e.g. prpc) is
// broken ?
// Option 1: Server shutdown the connection directly. Then client should (could)
//     realize that the connection was closed by the server. This connection shutdown
//     should be distinguished from a broken pipe (i.e. connection closed due to
//     unstable network) to implement correct retry policy.
// Option 2: Server shutdown the connection, and send a response message to state the
//     error. Drawbacks: server may be unable to know the correlation id, thus can not
//     make a targeted response. One work around is to add a special case to the
//     protocol (may be we can refer to an established protocol, like http?).

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

/// Client side codec
pub struct ProtoCodecClient {
    scheme: Box<RpcProtocol>,
}

impl fmt::Debug for ProtoCodecClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ProtoCodecClient")
            .field("scheme", &self.scheme.name())
            .finish()
    }
}

impl ProtoCodecClient {
    /// Create a new client codec.
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
