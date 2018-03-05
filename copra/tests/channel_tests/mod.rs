use bytes::{BigEndian, BufMut, BytesMut};
use copra::{ChannelBuilder, RpcErrorKind};
use copra::message::{RpcMeta, RpcResponseMeta};
use copra::controller::Controller;
use mock::MockServerBuilder;
use protobuf::{CodedOutputStream, Message};
use std::time::Duration;
use std::thread::spawn;
use tokio_core::reactor::{Core};

use generated::simple::Simple;
use generated::simple_copra::EchoStub;

fn simple(i: i32, b: bool, s: &str) -> Simple {
    let mut msg = Simple::new();
    msg.set_int_val(i);
    msg.set_bool_val(b);
    msg.set_str_val(s.to_string());

    msg
}

fn encode_message<T: Message>(msg: &T) -> BytesMut {
    let len = msg.compute_size() as usize;
    let mut bytes = BytesMut::with_capacity(len);
    unsafe {
        bytes.set_len(len);
    }
    {
        let mut os = CodedOutputStream::bytes(&mut bytes);
        msg.write_to_with_cached_sizes(&mut os).unwrap();
    }

    bytes
}

#[test]
fn success_echo() {
    let addr = "127.0.0.1:9001";

    let mut core = Core::new().unwrap();

    let mut builder = MockServerBuilder::new(addr, core.handle());

    let msg = simple(10, true, "HelloWorld");

    let send_msg = msg.clone();
    builder.respond_package(
        move || {
            let meta = RpcResponseMeta::new();
            let ctrl = Controller::default();
            (meta, ctrl, encode_message(&send_msg).freeze())
        },
        Duration::from_secs(0),
    );

    let join = spawn(move || {
        builder.build().start().unwrap();
    });

    let builder = ChannelBuilder::single_server(addr, core.handle());
    let channel = core.run(builder.build()).unwrap();
    let stub = EchoStub::new(&channel);

    let (resp, _info) = core.run(stub.echo(msg.clone())).unwrap();
    assert_eq!(resp, msg);

    join.join().unwrap();
}

#[test]
fn bad_body_message() {
    let addr = "127.0.0.1:9002";
    let mut core = Core::new().unwrap();

    let mut builder = MockServerBuilder::new(addr, core.handle());

    let msg = simple(10, true, "HelloWorld");

    let send_msg = msg.clone();
    builder.respond_package(
        move || {
            let meta = RpcResponseMeta::new();
            let ctrl = Controller::default();
            let mut body = encode_message(&send_msg);
            // distort body
            for e in body.iter_mut().take(4) {
                *e = 0;
            }
            (meta, ctrl, body.freeze())
        },
        Duration::from_secs(0),
    );

    let join = spawn(move || {
        builder.build().start().unwrap();
    });

    let builder = ChannelBuilder::single_server(addr, core.handle());
    let channel = core.run(builder.build()).unwrap();
    let stub = EchoStub::new(&channel);

    let result = core.run(stub.echo(msg));
    match result {
        Err(ref e) if e.kind() == RpcErrorKind::ResponseDecodeError => {}
        r @ _ => panic!("expect ResponseDecodeError, found {:?}", r),
    }

    join.join().unwrap();
}

#[test]
#[ignore]
fn bad_rpc_response_meta() {
    let addr = "127.0.0.1:9003";
    let mut core = Core::new().unwrap();

    let mut builder = MockServerBuilder::new(addr, core.handle());

    let msg = simple(10, true, "HelloWorld");

    let send_msg = msg.clone();
    builder.respond_bytes(
        move |id| {
            let resp_meta = RpcResponseMeta::new();
            let mut meta = RpcMeta::new();
            meta.set_correlation_id(id);
            meta.set_response(resp_meta);
            let mut meta_bytes = meta.write_to_bytes().unwrap();
            // distort meta
            for e in meta_bytes.iter_mut().take(4) {
                *e = 0;
            }
            let body = encode_message(&send_msg);

            let meta_len = meta_bytes.len() as u32;
            let body_len = body.len() as u32;
            let mut buf = BytesMut::with_capacity((meta_len + body_len + 12) as usize);

            buf.put_slice(b"PRPC");
            buf.put_u32::<BigEndian>(meta_len + body_len);
            buf.put_u32::<BigEndian>(meta_len);
            buf.put_slice(&meta_bytes);
            buf.put_slice(&body);

            buf.freeze()
        },
        Duration::from_secs(0),
    );

    let join = spawn(move || {
        builder.build().start().unwrap();
    });

    let builder = ChannelBuilder::single_server(addr, core.handle());
    let channel = core.run(builder.build()).unwrap();
    let stub = EchoStub::new(&channel);

    let result = core.run(stub.echo(msg));
    match result {
        Err(ref e) if e.kind() == RpcErrorKind::BrokenConnection => {}
        r @ _ => panic!("expect BrokenConnection, found {:?}", r),
    }

    join.join().unwrap();
}
