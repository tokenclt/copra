use bytes::Bytes;

use controller::Controller;

mod meta;

pub use self::meta::{RpcMeta, RpcRequestMeta, RpcResponseMeta};
pub type RequestPackage = (RpcRequestMeta, Controller, Bytes);
pub type ResponsePackage = (RpcResponseMeta, Controller, Bytes);
