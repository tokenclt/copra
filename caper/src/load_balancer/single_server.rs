use tokio_proto::multiplex::ClientService;
use std::sync::Arc;

use super::{CallInfo, LoadBalance, ServerId, ServerEndPort};

pub struct SingleServerLoadBalancer {
    service: ServerEndPort,
}

impl SingleServerLoadBalancer {
    pub fn new(service: ServerEndPort) -> Self {
        SingleServerLoadBalancer {
            service: service,
        }
    }
}

impl LoadBalance for SingleServerLoadBalancer {
    fn select_server(&mut self) -> (ServerId, &ServerEndPort) {
        (0, &self.service)
    }

    fn feed_back(&mut self, _: ServerId, _: CallInfo) {}
}
