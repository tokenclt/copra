extern crate caper;
extern crate futures;
extern crate protobuf;
extern crate tokio_proto;
extern crate tokio_service;

use futures::Future;
use tokio_service::{NewService, Service};
use proto::{EchoRequest, EchoResponse};
use caper::service::{EncapsulatedMethod, MethodError, NewEncapService, NewEncapsulatedMethod};
use caper::dispatcher::{Registrant, ServiceRegistry};
use caper::codec::{MethodCodec, ProtobufCodec};
use protobuf::Message;

mod proto;


trait EchoService {
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

struct EchoRegistrant<'a> {
    entries: Vec<(String, NewEncapService<'a>)>,
}

impl<'a> EchoRegistrant<'a> {
    pub fn new<S>(provider: S) -> Self
    where
        S: EchoService + Clone + 'a,
    {
        let mut entries = vec![];
        let wrap = EchoEchoWrapper__(provider.clone());
        let method = EncapsulatedMethod::new(ProtobufCodec::new(), wrap);
        entries.push((
            "echo".to_string(),
            Box::new(NewEncapsulatedMethod::new(method)) as NewEncapService,
        ));

        EchoRegistrant { entries }
    }
}

impl<'a> Registrant<'a> for EchoRegistrant<'a> {
    fn methods(&self) -> Vec<(String, NewEncapService<'a>)> {
        vec![]
    }
}


#[derive(Clone)]
struct Echo;

impl EchoService for Echo {
    type EchoFuture = Box<Future<Item = EchoResponse, Error = MethodError>>;

    type RevEchoFuture = Box<Future<Item = EchoResponse, Error = MethodError>>;

    fn echo(&self, msg: EchoRequest) -> Self::EchoFuture {
        unimplemented!()
    }

    fn rev_echo(&self, msg: EchoRequest) -> Self::RevEchoFuture {
        unimplemented!()
    }
}


fn main() {
    let registrant = EchoRegistrant::new(Echo);
    let mut registry = ServiceRegistry::new();
    registry.register_service(&"Echo".to_string(), registrant);


    println!("Hello from Echo.");
}
