extern crate caper;
extern crate futures;
extern crate protobuf;
extern crate tokio_proto;
extern crate tokio_service;

use futures::Future;
use tokio_service::{NewService, Service};
use proto::{EchoRequest, EchoResponse};
use caper::service::{EncapsulatedMethod, MethodError, NewEncapService};
use caper::dispatcher::Registrant;
use caper::codec::{MethodCodec, ProtobufCodec};
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

#[derive(Clone)]
struct EchoRegistrant<S: Clone> {
    provider: S,
}

impl<S: Clone> EchoRegistrant<S> {
    pub fn new(provider: S) -> Self {
        EchoRegistrant { provider }
    }
}

impl<'a, S> Registrant<'a> for EchoRegistrant<S>
where
    S: EchoService + Clone + 'a,
{
    fn methods(&self) -> Vec<(String, NewEncapService<'a>)> {
        let mut methods = vec![];
        let method = EncapsulatedMethod::new(
            ProtobufCodec::new(),
            EchoEchoWrapper__(self.provider.clone()),
        );
        methods.push(("echo".to_string(), Box::new(method) as NewEncapService));

        // let method = EncapsulatedMethod::new(ProtobufCodec::new(),
        //     EchoRevEchoWrapper__(self.clone());
        // methods.push(("rev_echo".to_string(), Box::new(method)));

        methods
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
    println!("Hello from Echo.");
}
