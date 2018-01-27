use bytes::Bytes;
use futures::{Async, Future, IntoFuture, Poll};
use std::io;

use codec::MethodCodec;
use channel::{Channel, ChannelError, ChannelFuture};
use load_balancer::CallInfo;
use message::{RpcRequestMeta, RpcResponseMeta};
use service::MethodError;

type ResponsePackage = (RpcResponseMeta, Bytes);

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
    pub fn call(&'a self, bundle: (C::Response, String, String)) -> StubFuture<C> {
        let (req, service_name, method_name) = bundle;
        let channel_fut = match self.codec.encode(req) {
            Ok(body) => {
                let mut meta = RpcRequestMeta::new();
                meta.set_service_name(service_name);
                meta.set_method_name(method_name);
                Some(self.channel.call((meta, body)))
            }
            Err(_) => None,
        };

        StubFuture::new(channel_fut, self.codec.clone())
    }
}

fn errno_to_result(result: ResponsePackage) -> Result<Bytes, MethodError> {
    let (meta, body) = result;
    let error_code = meta.get_error_code();
    if error_code == 0 {
        Ok(body)
    } else {
        error!("Server mark rpc to failed");
        Err(MethodError::UnknownError)
    }
}

pub struct StubFuture<C> {
    start_usec: u64,
    inner: Option<ChannelFuture>,
    codec: C,
}

impl<C> StubFuture<C> {
    pub fn new(inner: Option<ChannelFuture>, codec: C) -> Self {
        StubFuture {
            start_usec: 0,
            inner,
            codec,
        }
    }
}

impl<C> Future for StubFuture<C>
where
    C: MethodCodec,
{
    type Item = (C::Request, RpcInfo);

    type Error = MethodError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if let Some(ref mut channel) = self.inner {
            match channel.poll() {
                Ok(Async::Ready((resp, fb_handle))) => {
                    let body = errno_to_result(resp)?;
                    let resp = self.codec
                        .decode(body)
                        .map_err(|_| MethodError::CodecError)?;
                    let fb = CallInfo::new(self.start_usec, None);
                    let info = RpcInfo;
                    fb_handle.call(fb);

                    Ok(Async::Ready((resp, info)))
                }
                Ok(Async::NotReady) => Ok(Async::NotReady),
                // TODO: Add error convertion
                Err(_) => Err(MethodError::UnknownError),
            }
        } else {
            Err(MethodError::CodecError)
        }
    }
}

pub struct RpcInfo;
