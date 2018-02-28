extern crate bytes;
extern crate copra;
extern crate futures;
extern crate protobuf;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_timer;

pub mod generated;
pub mod mock;
pub mod channel_tests;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
