extern crate caper;
extern crate caper_examples;
extern crate env_logger;
extern crate futures;
extern crate tokio_core;

use caper::{Controller, MethodError, ServerBuilder, ServiceRegistry};
use caper::protocol::http::HttpStatus;
use futures::future::{self, FutureResult};
use std::mem::replace;

use caper_examples::protos::http_hello::{HelloRequest, HelloResponse};
use caper_examples::protos::http_hello_caper::{HelloRegistrant, HelloService};

#[derive(Clone)]
struct Hello;

impl HelloService for Hello {
    type HelloGeneralFuture = FutureResult<(HelloResponse, Controller), MethodError>;

    type HelloToFuture = FutureResult<(HelloResponse, Controller), MethodError>;

    fn hello_general(
        &self,
        (_, mut controller): (HelloRequest, Controller),
    ) -> Self::HelloGeneralFuture {
        let greeting = "Hello from the server.\n";
        controller.response_body = greeting.as_bytes().into();
        controller.status = Some(HttpStatus::Ok);
        controller.set_content_type("text/plain");

        future::ok((HelloResponse::new(), controller))
    }

    fn hello_to(&self, (_, mut controller): (HelloRequest, Controller)) -> Self::HelloToFuture {
        let raw = replace(&mut controller.request_body, Vec::new());
        let msg = String::from_utf8(raw);

        match msg {
            Ok(msg) => {
                let greeting = format!("Hello to {}.\n", msg);
                controller.response_body = greeting.into();
                controller.status = Some(HttpStatus::Ok);
                controller.set_content_type("text/plain");

                future::ok((HelloResponse::new(), controller))
            }
            Err(_) => future::err(MethodError::UnknownError),
        }
    }
}

fn main() {
    env_logger::init().unwrap();

    let addr = "127.0.0.1:8990";

    let registrant = HelloRegistrant::new(Hello);
    let mut registry = ServiceRegistry::new();
    registry.register_service(registrant);
    let server = ServerBuilder::new(addr, registry).build().unwrap();
    server.start();
}
