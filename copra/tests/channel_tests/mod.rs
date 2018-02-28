use bytes::BytesMut;
use copra::ChannelBuilder;
use copra::message::{ResponsePackage, RpcResponseMeta};
use copra::controller::Controller;
use futures::Future;
use mock::MockServerBuilder;
use protobuf::{CodedOutputStream, Message};
use std::time::Duration;
use std::thread::spawn;
use tokio_core::reactor::{Core, Handle};

use generated::simple::Simple;
use generated::simple_copra::EchoStub;

fn pack_response<T: Message>(meta: RpcResponseMeta, msg: &T, ctrl: Controller) -> ResponsePackage {
    let len = msg.compute_size() as usize;
    let mut bytes = BytesMut::with_capacity(len);
    unsafe {
        bytes.set_len(len);
    }
    {
        let mut os = CodedOutputStream::bytes(&mut bytes);
        msg.write_to_with_cached_sizes(&mut os).unwrap();
    }

    (meta, ctrl, bytes.freeze())
}

#[test]
fn success_echo() {
    let addr = "127.0.0.1:9001";

    let mut core = Core::new().unwrap();

    let mut builder = MockServerBuilder::new(addr, core.handle());

    let mut msg = Simple::new();
    msg.set_int_val(10);
    msg.set_bool_val(true);
    msg.set_str_val("HelloWorld".to_string());

    let send_msg = msg.clone();
    builder.respond_package(
        move || {
            let meta = RpcResponseMeta::new();
            let ctrl = Controller::default();
            pack_response(meta, &send_msg, ctrl)
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
