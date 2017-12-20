extern crate caper;
extern crate caper_examples;
extern crate env_logger;
extern crate futures;
extern crate protobuf;
extern crate tokio_core;
extern crate tokio_service;

use caper::controller::Controller;
use caper::channel::{ChannelBuilder, ChannelOption};
use caper::dispatcher::ServiceRegistry;
use caper::service::MethodError;
use caper::server::ServerBuilder;
use caper::protocol::http::HttpStatus;
use caper_examples::protos::benchmark::{Empty, PressureRequest, StringMessage};
use caper_examples::protos::benchmark_caper::{MetricRegistrant, MetricService, PressureRegistrant,
                                              PressureService, PressureStub};
use futures::Future;
use futures::future::FutureResult;
use futures::future;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio_core::reactor::Core;

#[derive(Clone)]
struct Pressure;

impl PressureService for Pressure {
    type EchoFuture = FutureResult<(StringMessage, Controller), MethodError>;

    type ProcessFuture = FutureResult<(Empty, Controller), MethodError>;

    fn echo(&self, msg: (StringMessage, Controller)) -> Self::EchoFuture {
        let (msg, controller) = msg;
        let string = msg.msg;
        let mut resp = StringMessage::new();
        resp.msg = string;
        future::ok((resp, controller))
    }

    fn process(&self, msg: (PressureRequest, Controller)) -> Self::ProcessFuture {
        unimplemented!()
    }
}

#[derive(Clone)]
struct Metric {
    throughput: Arc<AtomicUsize>,
}

impl Metric {
    pub fn new(throughput: Arc<AtomicUsize>) -> Self {
        Metric { throughput }
    }
}

impl MetricService for Metric {
    type MetricFuture = FutureResult<(Empty, Controller), MethodError>;

    fn metric(&self, msg: (Empty, Controller)) -> Self::MetricFuture {
        let (empty, mut controller) = msg;
        let throughput = self.throughput.load(Ordering::Relaxed);
        let resp = format!("Throughput: {}", throughput);
        controller.status = Some(HttpStatus::Ok);
        controller.response_body = resp.into();
        controller.set_content_type("text/plain");

        future::ok((empty, controller))
    }
}

fn main() {
    env_logger::init().unwrap();

    let client_thread_num = 4;
    let addr = "127.0.0.1:8991";
    let mut core = Core::new().unwrap();
    let mut registry = ServiceRegistry::new();
    let throughtput = Arc::new(AtomicUsize::new(0));

    let registrant = PressureRegistrant::new(Pressure);
    registry.register_service(&"Pressure".to_string(), registrant);
    let registrant = MetricRegistrant::new(Metric::new(throughtput.clone()));
    registry.register_service(&"Metric".to_string(), registrant);

    let server = ServerBuilder::new(addr, registry)
        .threads(2)
        .throughput(throughtput, core.remote())
        .build();

    thread::spawn(move || {
        server.start();
    });
    thread::sleep(Duration::from_millis(100));

    let _threads: Vec<_> = (0..client_thread_num)
        .map(|_| {
            thread::spawn(move || {
                let channel_option = ChannelOption::new();
                let mut core = Core::new().unwrap();
                let handle = core.handle();
                let (channel, backend) = core.run(ChannelBuilder::single_server(
                    addr,
                    handle.clone(),
                    channel_option,
                )).unwrap();

                handle.spawn(backend);

                let pressure = PressureStub::new(&channel);
                loop {
                    let mut req = StringMessage::new();
                    req.set_msg("ABCDE_ABCDE_ABCDE_ABCDE_ABCDE_ABCDE_ABCDE_ABCDE".to_string());
                    let resp = pressure.echo(req);
                    core.run(resp).unwrap();
                }
            })
        })
        .collect();

    core.run(future::empty::<(), ()>()).unwrap();
}
