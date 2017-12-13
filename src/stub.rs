use bytes::BytesMut;
use futures::{Future, IntoFuture};
use tokio_service::Service;
use std::marker::PhantomData;

use codec::MethodCodec;
use channel::Channel;
use protocol::Meta;
use service::MethodError;

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
        let request_body = self.codec.encode(req).into_future();

        let response = request_body
            .map_err(|_| MethodError::UnknownError)
            .and_then(move |body| {
                let meta = Meta {
                    body,
                    service_name,
                    method_name,
                };
                self.channel.call(meta)
            })
            .and_then(move |meta| {
                self.codec
                    .decode(meta.body)
                    .map_err(|_| MethodError::UnknownError)
            })
            .map(|resp| (resp, RpcInfo));

        Box::new(response)
    }
}


pub struct Retry;

pub struct RpcOption;

pub struct RpcInfo;
