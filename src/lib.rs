extern crate bytes;
extern crate futures;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate smallvec;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

pub mod codec;
pub mod protocol;
pub mod service;
pub mod dispatcher;

use std::io;
use std::str;
use bytes::BytesMut;

use tokio_service::Service;

