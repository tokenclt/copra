use tokio_core::net::TcpStream;
use tokio_proto::multiplex::ClientService;
use tokio_service::Service;

use channel::{MetaClientProtocol, RequestPackage, ResponsePackage};
use service::MethodError;

pub mod single_server;

type InnerService = ClientService<TcpStream, MetaClientProtocol>;

pub type ServerId = u64;

pub struct ServerEndPort(InnerService);

impl ServerEndPort {
    pub(crate) fn new(service: InnerService) -> Self {
        ServerEndPort(service)
    }
}

impl Service for ServerEndPort {
    type Request = <InnerService as Service>::Request;
    type Response = <InnerService as Service>::Response;
    type Error = <InnerService as Service>::Error;
    type Future = <InnerService as Service>::Future;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.0.call(req)
    }
}

#[derive(Clone, Debug, Default)]
pub struct CallInfo {
    pub error: Option<MethodError>,
    pub start_usec: u64,
}

impl CallInfo {
    pub fn new(start_usec: u64, error: Option<MethodError>) -> Self {
        CallInfo { start_usec, error }
    }
}

pub trait LoadBalance {
    fn select_server(&mut self) -> (ServerId, &ServerEndPort);

    fn feed_back(&mut self, id: ServerId, call_info: CallInfo);
}
