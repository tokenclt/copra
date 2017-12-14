use tokio_proto::TcpServer;
use tokio_proto::multiplex::{Multiplex, ServerProto};
use tokio_core::net::TcpStream;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_service::{Service, NewService};
use std::io;
use std::net::SocketAddr;
use futures::{Future, IntoFuture};

use protocol::{BrpcProtocol, Meta, ProtoCodec, Protocol, RpcProtocol};
use dispatcher::ServiceRegistry;
use service::{EncapService, MethodError, MethodFuture};

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
    type Request = Meta;
    type Response = Meta;
    type Transport = Framed<T, ProtoCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let codec = ProtoCodec::with_protocols(self.protocols.as_slice());
        Ok(io.framed(codec))
    }
}

pub struct MetaService<'a, 'b: 'a> {
    registry: &'a ServiceRegistry<'b>,
}

impl<'a, 'b: 'a> MetaService<'a, 'b> {
    pub fn new(registry: &'a ServiceRegistry<'b>) -> Self {
        MetaService { registry }
    }
}

impl<'a, 'b: 'a> Service for MetaService<'a, 'b> {
    type Request = Meta;
    type Response = Meta;
    type Error = MethodError;
    type Future = MethodFuture<'b>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let service = {
            let service_name = &req.service_name;
            let method_name = &req.method_name;
            self.registry
                .get_method(service_name, method_name)
                .ok_or(MethodError::UnknownError)
                .into_future()
        };
        let response = service.and_then(|service| service.call(req));
        Box::new(response)
    }
}


pub struct Server<'a> {
    services: ServiceRegistry<'a>,
    listener: TcpServer<Multiplex, MetaServerProtocol>,
}

impl<'a> Server<'a> {
    pub fn new(addr: &str, option: &ServerOption) -> Self {
        let socket_addr = addr.parse().expect("Parse listening addr failed");
        Server {
            services: ServiceRegistry::new(),
            listener: TcpServer::new(MetaServerProtocol::new(option), socket_addr),
        }
    }

    pub fn start(&self) {
        unimplemented!()
    }
}
