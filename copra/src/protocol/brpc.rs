//! Brpc protocol, inspired by [brpc] framework developed by Baidu Inc.
//! 
//! [brpc]: https://github.com/brpc/brpc

use bytes::{BigEndian, Buf, BufMut, Bytes, BytesMut, IntoBuf};
use std::io;
use tokio_proto::multiplex::RequestId;
use protobuf::{parse_from_carllerche_bytes, Message};

use super::{ProtocolError, RpcProtocol};
use controller::Controller;
use message::RpcMeta;

static HEADER: &[u8] = b"PRPC";

#[derive(Clone, Debug)]
enum BrpcParseState {
    ReadingHeader,
    ReadingLength,
    ReadingContent(u32, u32),
}


/// Brpc protocol
#[derive(Clone, Debug)]
pub struct BrpcProtocol {
    state: BrpcParseState,
}

impl BrpcProtocol {
    /// Create a new instance.
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
                    let pkg_len = buf.split_to(4).into_buf().get_u32::<BigEndian>();
                    let meta_len = buf.split_to(4).into_buf().get_u32::<BigEndian>();
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
        buf.reserve(pkg_len as usize);

        debug_assert!(HEADER.len() == 4);
        buf.put_slice(HEADER);
        buf.put_u32::<BigEndian>(meta_len + body_len as u32);
        buf.put_u32::<BigEndian>(meta_len);
        // TODO remove copy
        buf.put_slice(meta.write_to_bytes()?.as_slice());
        buf.put(body);

        Ok(())
    }

    fn name(&self) -> &'static str {
        "brpc"
    }
}

#[cfg(test)]
mod test {
    use rand::{Rng, SeedableRng, XorShiftRng};
    use protobuf::CodedOutputStream;

    use super::*;
    use message::{RpcRequestMeta, TestMessage};
    // TODO: use a simpler random engine

    const CORRELATION_ID: u64 = 10_u64;
    const SERVICE: &'static str = "TestService";
    const METHOD: &'static str = "TestMethod";
    const SEED: [u32; 4] = [100, 200, 300, 400];

    fn get_test_message() -> TestMessage {
        let mut msg = TestMessage::new();
        msg.set_i32_field(1234567_i32);
        msg.set_float_field(123.456789_f32);
        msg.set_string_field("testing_testing".to_string());

        msg
    }

    fn convert_to_bytes<T>(msg: T) -> Bytes
    where
        T: Message,
    {
        let len = msg.compute_size();

        let mut buf = BytesMut::with_capacity(len as usize);
        // CodedOutputStream counts length, not capacity.
        unsafe {
            buf.set_len(len as usize);
        }
        {
            let mut os = CodedOutputStream::bytes(buf.as_mut());
            msg.write_to_with_cached_sizes(&mut os)
                .expect("failed to serialize protobuf data");
        }

        buf.freeze()
    }

    #[test]
    fn split_randomly_to_simulate_incomplete_receive() {
        let mut brpc_codec = BrpcProtocol::new();

        let mut request_meta = RpcRequestMeta::new();
        request_meta.set_service_name(SERVICE.to_string());
        request_meta.set_method_name(METHOD.to_string());

        let mut meta = RpcMeta::new();
        meta.set_correlation_id(CORRELATION_ID);
        meta.set_request(request_meta);

        let controller = Controller::default();
        let body = get_test_message();
        let body_bytes = convert_to_bytes(body.clone());

        let mut buf = BytesMut::new();
        brpc_codec
            .write_package((meta.clone(), controller.clone(), body_bytes.clone()), &mut buf)
            .expect("write_package failed");

        let max_segment_num = 5;
        let repeat_num = 5;

        let mut rng = XorShiftRng::from_seed(SEED.clone());
        for segment_num in 1..max_segment_num {
            for _ in 0..repeat_num {
                let mut unfeeded_bytes = buf.clone();
                let mut simulated_buf = BytesMut::with_capacity(unfeeded_bytes.len());

                for seg_id in 1..segment_num {
                    let len = unfeeded_bytes.len();
                    let split_point = rng.gen_range::<usize>(1, len + seg_id + 1 - segment_num);
                    let feed = unfeeded_bytes.split_to(split_point);
                    simulated_buf.put_slice(feed.as_ref());

                    let result = brpc_codec.try_parse(&mut simulated_buf);
                    assert_eq!(result, Err(ProtocolError::NeedMoreBytes));
                }

                simulated_buf.put_slice(unfeeded_bytes.as_ref());
                let result = brpc_codec.try_parse(&mut simulated_buf);
                assert_eq!(result, Ok((CORRELATION_ID, (meta.clone(), controller.clone(), body_bytes.clone()))));
            }
        }
    }
}
