use bytes::{BigEndian, Buf, BufMut, Bytes, BytesMut};
use futures::Future;
use std::io;
use tokio_io::codec::{Decoder, Encoder};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_proto::multiplex::RequestId;
use protobuf::{parse_from_carllerche_bytes, Message};

use super::{ProtocolError, RpcProtocol};
use controller::Controller;
use message::{RpcMeta, RpcRequestMeta, RpcResponseMeta};
use message::{RequestPackage, ResponsePackage};

static HEADER: &[u8] = b"PRPC";

#[derive(Clone)]
enum BrpcParseState {
    ReadingHeader,
    ReadingLength,
    ReadingContent(u32, u32),
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

    fn new_boxed(&self) -> Box<RpcProtocol> {
        Box::new(BrpcProtocol {
            state: BrpcParseState::ReadingHeader,
        })
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
