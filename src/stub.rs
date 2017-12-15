use bytes::{Bytes, BytesMut};
use futures::{Future, IntoFuture};
use tokio_service::Service;
use std::marker::PhantomData;
use std::io;

use codec::MethodCodec;
use channel::Channel;

use message::{RpcRequestMeta, RpcResponseMeta};
use service::MethodError;

type ResponsePackage = (RpcResponseMeta, Bytes);

pub type StubCallFuture<'a, T> = Box<Future<Item = (T, RpcInfo), Error = MethodError> + 'a>;

#[derive(Clone)]
pub struct RpcWrapper<'a, C: Clone> {
    codec: C,
    channel: &'a Channel,
}

impl<'a, C: Clone> RpcWrapper<'a, C> {
    pub fn new(codec: C, channel: &'a Channel) -> Self {
        RpcWrapper { codec, channel }
    }
}

impl<'a, C> RpcWrapper<'a, C>
where
    C: MethodCodec + Clone,
{
    // inverse of request and response
    pub fn call(&'a self, bundle: (C::Response, String, String)) -> StubCallFuture<'a, C::Request> {
        let (req, service_name, method_name) = bundle;
        let body = self.codec.encode(req).into_future();

        let response = body.map_err(|_| MethodError::UnknownError)
            .and_then(move |body| {
                let mut meta = RpcRequestMeta::new();
                meta.set_service_name(service_name);
                meta.set_method_name(method_name);
                self.channel
                    .call((meta, body))
                    .then(|resp| errno_to_result(resp))
            })
            .and_then(move |body| {
                self.codec
                    .decode(body)
                    .map_err(|_| MethodError::UnknownError)
            })
            .map(|resp| (resp, RpcInfo));

        Box::new(response)
    }
}

fn errno_to_result(result: io::Result<ResponsePackage>) -> Result<Bytes, MethodError> {
    result
        .map_err(|_| MethodError::UnknownError)
        .and_then(|(meta, body)| {
            let error_code = meta.get_error_code();
            if error_code == 0 {
                Ok(body)
            } else {
                Err(MethodError::UnknownError)
            }
        })
}


pub struct Retry;

pub struct RpcOption;

pub struct RpcInfo;
