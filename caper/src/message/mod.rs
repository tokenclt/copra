//! Resquest and response messages

use bytes::Bytes;

use controller::Controller;

mod meta;
mod test;

pub use self::meta::{RpcMeta, RpcRequestMeta, RpcResponseMeta};

/// RPC request headers and parameters
pub type RequestPackage = (RpcRequestMeta, Controller, Bytes);
/// RPC response headers and body
pub type ResponsePackage = (RpcResponseMeta, Controller, Bytes);

#[cfg(test)]
pub(crate) use self::test::TestMessage;
