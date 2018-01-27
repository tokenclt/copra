use tokio_core::net::TcpStream;
use tokio_proto::multiplex::ClientService;

use channel::MetaClientProtocol;
use channel::connector::Connector;
use service::MethodError;

pub mod single_server;

pub type ServerId = u64;
pub type ServerEndPort = ClientService<TcpStream, MetaClientProtocol>;

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
