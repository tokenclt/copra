extern crate bytes;
#[macro_use]
extern crate futures;
extern crate httparse;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate smallvec;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate url;

pub mod channel;
pub mod controller;
pub mod codec;
pub mod protocol;
pub mod service;
pub mod stub;
pub mod dispatcher;
pub mod server;
pub mod message;

use std::io;
use std::str;
use bytes::BytesMut;

use tokio_service::Service;

