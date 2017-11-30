extern crate caper;
extern crate futures;
extern crate protobuf;
extern crate tokio_proto;
extern crate tokio_service;

use futures::Future;
use tokio_service::{NewService, Service};
use proto::{EchoRequest, EchoResponse};
use caper::service::{EncapsulatedMethod, MethodError};
use caper::dispatcher::{NewEncapService, Registrant};
use caper::codec::{ProtobufCodec, MethodCodec};
use protobuf::Message;

mod proto;


trait EchoService {
    type EchoFuture: Future<Item = EchoResponse, Error = MethodError>;
    type RevEchoFuture: Future<Item = EchoResponse, Error = MethodError>;

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
    type Future = <Self as EchoService>::EchoFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        // TODO: async decoding in CPU pool.
        // TODO: create a dedicated wrapper struct in main crate.
        self.0.echo(req)
    }
}

impl<S: Clone + EchoService> NewService for EchoEchoWrapper__<S> {
    type Request = EchoRequest;
    type Response = EchoResponse;
    type Error = MethodError;
    type Instance = EncapService;

    fn new_service(&self) -> std::io::Result<Self> {
        Ok(Box::new(self.clone()))
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
struct EchoRevEchoWrapper__<S: Clone>(S);

impl<S: EchoService + Clone> Service for EchoRevEchoWrapper__<S> {
    type Request = EchoRequest;
    type Response = EchoResponse;
    type Error = MethodError;
    type Future = <Self as EchoService>::RevEchoFuture;

    fn call(&self, req: Self::Request) -> Self::Response {
        self.0.rev_echo(req)
    }
}

impl<S: Clone + EchoService> NewService for EchoRevEchoWrapper__<S> {
    type Request = EchoRequest;
    type Response = EchoResponse;
    type Error = MethodError;
    type Instance = Self;

    fn new_service(&self) -> std::io::Result<Self> {
        Ok(Box::new(self.clone()))
    }
}

struct EchoRegistrant<C, S> {
    codec: C,
    provider: S,
}

impl<C, S> EchoRegistrant<C, S> {
    pub fn new(codec: C, provider: S) -> Self {
        EchoRegistrant{codec, provider}
    }
}

impl<S: Clone + EchoService> Registrant for EchoRegistrant<S> {
    fn methods(&self) -> Vec<(String, NewEncapService)> {
        let methods = vec![];
        let encap = EncapsulatedMethod::new()
        methods.push((
            "echo".to_string(),
            Box::new(EchoEchoWrapper__(self.0.clone())),
        ));
        methods.push((
            "rev_echo".to_string(),
            Box::new(EchoRevEchoWrapper__(self.0.clone)),
        ));
        methods
    }
}

#[derive(Clone)]
struct Echo;

impl EchoService for Echo {
    type EchoFuture = Box<Future<Item = EchoRequest, Error = MethodError>>;

    type RevEchoFuture = Box<Future<Item = EchoRequest, Error = MethodError>>;

    fn echo(&self, msg: EchoRequest) -> Self::EchoFuture {
        unimplemented!()
    }

    fn rev_echo(&self, msg: EchoRequest) -> Self::RevEchoFuture {
        unimplemented!()
    }
}


fn main() {
    println!("Hello from Echo.");
}
