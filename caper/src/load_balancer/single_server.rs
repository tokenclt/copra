//! Load balancing for a single server

use super::{CallInfo, LoadBalance, ServerId, ServerEndPort};

/// Provide load balancing for a single server
#[derive(Debug)]
pub struct SingleServerLoadBalancer {
    service: ServerEndPort,
}

impl SingleServerLoadBalancer {
    /// Create a new instance
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
