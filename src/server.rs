use bytes::Bytes;
use tokio_proto::TcpServer;
use tokio_proto::multiplex::{Multiplex, ServerProto};
use tokio_core::net::TcpStream;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_service::{NewService, Service};
use std::io;
use std::sync::Arc;
use std::net::SocketAddr;
use futures::{Future, IntoFuture};

use protocol::{BrpcProtocol, ProtoCodec, Protocol, RpcProtocol};
use dispatcher::ServiceRegistry;
use service::{EncapService, MethodError, MethodFuture};
use message::{RpcRequestMeta, RpcResponseMeta};

type RequestPackage = (RpcRequestMeta, Bytes);
type ResponsePackage = (RpcResponseMeta, Bytes);

pub type MetaServiceFuture = Box<Future<Item = ResponsePackage, Error = io::Error>>;

#[derive(Clone)]
pub struct ServerOption {
    pub protocols: Vec<Protocol>,
}

pub struct MetaServerProtocol {
    protocols: Vec<Box<RpcProtocol>>,
}

impl MetaServerProtocol {
    pub fn new(option: &ServerOption) -> Self {
        let protocols: Vec<_> = option
            .protocols
            .iter()
            .map(|proto| match proto {
                &Protocol::Brpc => Box::new(BrpcProtocol) as Box<RpcProtocol>,
                _ => unimplemented!(),
            })
            .collect();
        MetaServerProtocol { protocols }
    }
}

impl<T> ServerProto<T> for MetaServerProtocol
where
    T: AsyncRead + AsyncWrite + 'static,
{
    type Request = RequestPackage;
    type Response = ResponsePackage;
    type Transport = Framed<T, ProtoCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let codec = ProtoCodec::new(self.protocols.as_slice());
        Ok(io.framed(codec))
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
        let (meta, body) = req;
        let service = {
            let service_name = meta.get_service_name();
            let method_name = meta.get_method_name();
            self.registry
                .get_method(service_name, method_name)
                .ok_or(MethodError::UnknownError)
                .into_future()
        };
        let response = service
            .and_then(|service| service.call(body))
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


pub struct Server {
    services: Arc<ServiceRegistry>,
    listener: TcpServer<Multiplex, MetaServerProtocol>,
}

impl Server {
    pub fn new(addr: &str, option: ServerOption) -> Self {
        let socket_addr = addr.parse().expect("Parse listening addr failed");
        Server {
            services: Arc::new(ServiceRegistry::new()),
            listener: TcpServer::new(MetaServerProtocol::new(&option), socket_addr),
        }
    }

    pub fn start(&self) {
        self.listener.serve(MetaService::new(self.services.clone()))
    }
}

fn result_to_errno(result: Result<Bytes, MethodError>) -> io::Result<ResponsePackage> {
    result
        .and_then(|body| {
            let mut meta = RpcResponseMeta::new();
            meta.set_error_code(0);
            Ok((meta, body))
        })
        .or_else(|_| {
            let mut meta = RpcResponseMeta::new();
            meta.set_error_code(1);
            meta.set_error_text("Unknown error".to_string());
            Ok((meta, Bytes::new()))
        })
}
