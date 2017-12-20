use bytes::Bytes;
use tokio_core::reactor::Remote;
use tokio_proto::TcpServer;
use tokio_proto::multiplex::{Multiplex, ServerProto};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_service::{NewService, Service};
use std::io;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use futures::{Future, IntoFuture, Stream};
use futures::future::Executor;

use controller::Controller;
use protocol::{BrpcProtocol, HttpProtocol, ProtoCodec, Protocol, RpcProtocol};
use dispatcher::ServiceRegistry;
use service::MethodError;
use message::RpcResponseMeta;
use message::{RequestPackage, ResponsePackage};
use monitor::{ThroughputMaintainer, TrafficCounting};

pub type MetaServiceFuture = Box<Future<Item = ResponsePackage, Error = io::Error>>;

#[derive(Clone)]
pub struct ServerOption {
    pub protocols: Vec<Protocol>,
}

impl Default for ServerOption {
    fn default() -> Self {
        ServerOption {
            protocols: vec![Protocol::Brpc, Protocol::Http],
        }
    }
}

pub struct MetaServerProtocol {
    protocols: Vec<Box<RpcProtocol>>,
    finished: Arc<AtomicUsize>,
}

impl MetaServerProtocol {
    pub fn new(option: &ServerOption, finished: Arc<AtomicUsize>) -> Self {
        let protocols: Vec<_> = option
            .protocols
            .iter()
            .map(|proto| match proto {
                &Protocol::Brpc => Box::new(BrpcProtocol::new()) as Box<RpcProtocol>,
                &Protocol::Http => Box::new(HttpProtocol::new()) as Box<RpcProtocol>,
            })
            .collect();

        MetaServerProtocol {
            protocols,
            finished,
        }
    }
}

impl<T> ServerProto<T> for MetaServerProtocol
where
    T: AsyncRead + AsyncWrite + 'static,
{
    type Request = RequestPackage;
    type Response = ResponsePackage;
    type Transport = TrafficCounting<Framed<T, ProtoCodec>>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        debug!("New connection established");
        let codec = ProtoCodec::new(self.protocols.as_slice());
        let counting_wrapper = TrafficCounting::new(self.finished.clone(), io.framed(codec));
        Ok(counting_wrapper)
    }
}

#[derive(Clone)]
pub struct MetaService {
    registry: Arc<ServiceRegistry>,
}

impl MetaService {
    pub fn new(registry: Arc<ServiceRegistry>) -> Self {
        MetaService { registry }
    }
}

impl Service for MetaService {
    type Request = RequestPackage;
    type Response = ResponsePackage;
    type Error = io::Error;
    type Future = MetaServiceFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        let (meta, controller, body) = req;
        let service = {
            let service_name = meta.get_service_name();
            let method_name = meta.get_method_name();
            self.registry
                .get_method(service_name, method_name)
                .ok_or(MethodError::UnknownError)
                .map_err(|e| {
                    warn!(
                        "Requested method {}::{} is not found",
                        service_name, method_name
                    );
                    e
                })
                .into_future()
        };
        let response = service
            .and_then(|service| service.call((body, controller)))
            .then(|resp| result_to_errno(resp));
        Box::new(response)
    }
}

impl NewService for MetaService {
    type Request = RequestPackage;
    type Response = ResponsePackage;
    type Error = io::Error;
    type Instance = Self;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

pub struct ServerBuilder<'a> {
    services: ServiceRegistry,
    addr: &'a str,
    threads: Option<usize>,
    option: Option<ServerOption>,
    remote: Option<Remote>,
    throughput: Option<Arc<AtomicUsize>>,
}

impl<'a> ServerBuilder<'a> {
    pub fn new(addr: &'a str, services: ServiceRegistry) -> Self {
        ServerBuilder {
            services,
            addr,
            threads: None,
            option: None,
            remote: None,
            throughput: None,
        }
    }

    pub fn threads(mut self, threads: usize) -> Self {
        self.threads = Some(threads);
        self
    }

    pub fn option(mut self, option: ServerOption) -> Self {
        self.option = Some(option);
        self
    }

    pub fn throughput(mut self, throughput: Arc<AtomicUsize>, remote: Remote) -> Self {
        self.throughput = Some(throughput);
        self.remote = Some(remote);
        self
    }

    pub fn build(self) -> Server {
        let finished = Arc::new(AtomicUsize::new(0));
        let threads = self.threads.unwrap_or(1);
        let option = self.option.unwrap_or(ServerOption::default());
        let throughput = self.throughput.unwrap_or(Arc::new(AtomicUsize::new(0)));
        let socket_addr = self.addr.parse().expect("Parse listening addr failed");
        let mut server = TcpServer::new(
            MetaServerProtocol::new(&option, finished.clone()),
            socket_addr,
        );
        server.threads(threads);
        
        info!("Server listensing : {}", socket_addr);
        Server {
            services: Arc::new(self.services),
            listener: server,
            throughput,
            finished,
            remote: self.remote,
        }
    }
}

pub struct Server {
    services: Arc<ServiceRegistry>,
    listener: TcpServer<Multiplex, MetaServerProtocol>,
    finished: Arc<AtomicUsize>,
    throughput: Arc<AtomicUsize>,
    remote: Option<Remote>,
}

impl Server {
    pub fn start(&self) {
        if let Some(ref remote) = self.remote {
            let maintainer =
                ThroughputMaintainer::new(self.finished.clone(), self.throughput.clone());
            remote.execute(maintainer.for_each(|_| Ok(()))).unwrap();
        }

        self.listener.serve(MetaService::new(self.services.clone()))
    }
}

fn result_to_errno(
    result: Result<(Bytes, Controller), MethodError>,
) -> io::Result<ResponsePackage> {
    result
        .and_then(|(body, controller)| {
            let mut meta = RpcResponseMeta::new();
            meta.set_error_code(0);
            Ok((meta, controller, body))
        })
        .or_else(|_| {
            let mut meta = RpcResponseMeta::new();
            meta.set_error_code(1);
            meta.set_error_text("Unknown error".to_string());
            Ok((meta, Controller::default(), Bytes::new()))
        })
}
