//! [WIP] Load balancer traits and algorithms

use tokio_core::net::TcpStream;
use tokio_proto::multiplex::ClientService;
use tokio_service::Service;

use channel::MetaClientProtocol;
use service::MethodError;

pub mod single_server;

type InnerService = ClientService<TcpStream, MetaClientProtocol>;

/// Server ID
pub type ServerId = u64;


/// Represent a load lalancing unit
#[derive(Debug)]
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

/// Information needed by the load lalancer to adjust algorithm
#[derive(Clone, Debug, Default)]
pub struct CallInfo {
    /// If any error raised when processing a request
    pub error: Option<MethodError>,
    /// When the request started
    pub start_usec: u64,
}

impl CallInfo {
    /// Create a new instance.
    pub fn new(start_usec: u64, error: Option<MethodError>) -> Self {
        CallInfo { start_usec, error }
    }
}


/// Something can serve as a load balancer
pub trait LoadBalance {
    /// Select a server to send request.
    fn select_server(&mut self) -> (ServerId, &ServerEndPort);

    /// Update load balancing state.
    fn feed_back(&mut self, id: ServerId, call_info: CallInfo);
}
