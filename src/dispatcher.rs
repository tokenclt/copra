use bytes::BytesMut;
use std::collections::HashMap;
use tokio_service::{NewService, Service};

use service::{MethodError, MethodFuture};
use protocol::Meta;

pub type EncapService = Box<
    Service<Request = Meta, Response = BytesMut, Error = MethodError, Future = MethodFuture>,
>;

pub type NewEncapService = Box<
    NewService<Request = Meta, Response = BytesMut, Error = MethodError, Instance = EncapService>,
>;

pub struct ServiceRegistry {
    registry: HashMap<String, HashMap<String, Box<NewEncapService>>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        ServiceRegistry {
            registry: HashMap::new(),
        }
    }

    pub fn register_service<T>(&mut self, service_name: &String, registrant: T)
    where
        T: Registrant,
    {
        let mut map = HashMap::new();
        for (method_name, encap) in registrant.methods().into_iter() {
            map.insert(method_name, encap);
        }
        self.registry.insert(service_name.clone(), map);
    }

    pub fn get_method(&self, service_name: &String, method_name: &String) -> Option<EncapService> {
        self.registry
            .get(service_name)
            .and_then(|methods| methods.get(method_name))
            .map(|s| s.new_service().unwrap())
    }
}

pub trait Registrant: Clone {
    fn methods(&self) -> Vec<(String, Box<NewEncapService>)>;
}
