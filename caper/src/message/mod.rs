use bytes::Bytes;

use controller::Controller;

mod meta;
mod test;

pub use self::meta::{RpcMeta, RpcRequestMeta, RpcResponseMeta};
pub type RequestPackage = (RpcRequestMeta, Controller, Bytes);
pub type ResponsePackage = (RpcResponseMeta, Controller, Bytes);

#[cfg(test)]
pub(crate) use self::test::TestMessage;
