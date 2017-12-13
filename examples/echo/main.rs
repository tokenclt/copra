extern crate caper;
extern crate futures;
extern crate protobuf;
extern crate tokio_proto;
extern crate tokio_service;

use futures::{Future, IntoFuture};
use tokio_service::{NewService, Service};
use proto::{EchoRequest, EchoResponse};
use caper::service::{EncapsulatedMethod, MethodError, NewEncapService, NewEncapsulatedMethod};
use caper::dispatcher::{Registrant, ServiceRegistry};
use caper::codec::{MethodCodec, ProtobufCodec};
use caper::channel::Channel;
use caper::stub::{RpcWrapper, StubCallFuture};
use protobuf::Message;

mod proto;

pub trait EchoService {
    type EchoFuture: Future<Item = EchoResponse, Error = MethodError> + 'static;
    type RevEchoFuture: Future<Item = EchoResponse, Error = MethodError> + 'static;

    fn echo(&self, msg: EchoRequest) -> Self::EchoFuture;

    fn rev_echo(&self, msg: EchoRequest) -> Self::RevEchoFuture;
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
struct EchoEchoWrapper__<S: Clone>(S);

impl<S: EchoService + Clone> Service for EchoEchoWrapper__<S> {
    type Request = EchoRequest;
    type Response = EchoResponse;
    type Error = MethodError;
    type Future = <S as EchoService>::EchoFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.0.echo(req)
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
struct EchoRevEchoWrapper__<S: Clone>(S);

impl<S: EchoService + Clone> Service for EchoRevEchoWrapper__<S> {
    type Request = EchoRequest;
    type Response = EchoResponse;
    type Error = MethodError;
    type Future = <S as EchoService>::RevEchoFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.0.rev_echo(req)
    }
}

pub struct EchoRegistrant<'a, S: 'a> {
    provider: S,
    phantom: ::std::marker::PhantomData<&'a ()>,
}

impl<'a, S: 'a> EchoRegistrant<'a, S> {
    pub fn new(provider: S) -> Self {
        EchoRegistrant {
            provider,
            phantom: ::std::marker::PhantomData,
        }
    }
}

impl<'a, S> Registrant<'a> for EchoRegistrant<'a, S>
where
    S: EchoService + Clone + 'a,
{
    fn methods(&self) -> Vec<(String, NewEncapService<'a>)> {
        let mut entries = vec![];
        let provider = &self.provider;

        let wrap = EchoEchoWrapper__(provider.clone());
        let method = EncapsulatedMethod::new(ProtobufCodec::new(), wrap);
        entries.push((
            "echo".to_string(),
            Box::new(NewEncapsulatedMethod::new(method)) as NewEncapService,
        ));

        let wrap = EchoRevEchoWrapper__(provider.clone());
        let method = EncapsulatedMethod::new(ProtobufCodec::new(), wrap);
        entries.push((
            "rev_echo".to_string(),
            Box::new(NewEncapsulatedMethod::new(method)) as NewEncapService,
        ));

        entries
    }
}

#[derive(Clone)]
pub struct EchoStub<'a> {
    echo_wrapper: RpcWrapper<'a, ProtobufCodec<EchoResponse, EchoRequest>>,
    rev_echo_wrapper: RpcWrapper<'a, ProtobufCodec<EchoResponse, EchoRequest>>,
}

impl<'a> EchoStub<'a> {
    pub fn new(channel: &'a Channel) -> Self {
        EchoStub {
            echo_wrapper: RpcWrapper::new(ProtobufCodec::new(), channel),
            rev_echo_wrapper: RpcWrapper::new(ProtobufCodec::new(), channel),
        }
    }

    pub fn echo(&'a self, msg: EchoRequest) -> StubCallFuture<'a, EchoResponse> {
        self.echo_wrapper
            .call((msg, "Echo".to_string(), "echo".to_string()))
    }

    pub fn rev_echo(&'a self, msg: EchoRequest) -> StubCallFuture<'a, EchoResponse> {
        self.rev_echo_wrapper
            .call((msg, "Echo".to_string(), "rev_echo".to_string()))
    }
}


// user visible from here

#[derive(Clone)]
struct Echo;

impl EchoService for Echo {
    type EchoFuture = Box<Future<Item = EchoResponse, Error = MethodError>>;

    type RevEchoFuture = Box<Future<Item = EchoResponse, Error = MethodError>>;

    fn echo(&self, msg: EchoRequest) -> Self::EchoFuture {
        let string = msg.msg;
        let mut response = EchoResponse::new();
        response.msg = string;
        let future = Ok(response).into_future();

        Box::new(future)
    }

    fn rev_echo(&self, msg: EchoRequest) -> Self::RevEchoFuture {
        let string = msg.msg;
        let rev: String = string.chars().rev().collect();
        let mut response = EchoResponse::new();
        response.msg = rev;
        let future = Ok(response).into_future();

        Box::new(future)
    }
}


fn main() {
    let registrant = EchoRegistrant::new(Echo);
    let mut registry = ServiceRegistry::new();
    registry.register_service(&"Echo".to_string(), registrant);


    println!("Hello from Echo.");
}
