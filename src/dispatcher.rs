use bytes::BytesMut;
use std::collections::HashMap;
use tokio_service::{NewService, Service};

use service::{EncapService, MethodError, MethodFuture, NewEncapService};
use protocol::Meta;



pub struct ServiceRegistry<'a> {
    registry: HashMap<String, HashMap<String, NewEncapService<'a>>>,
}

impl<'a> ServiceRegistry<'a> {
    pub fn new() -> Self {
        ServiceRegistry {
            registry: HashMap::new(),
        }
    }

    pub fn register_service<T>(&mut self, service_name: &String, registrant: T)
    where
        T: Registrant<'a>,
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

pub trait Registrant<'a> {
    fn methods(&self) -> Vec<(String, NewEncapService<'a>)>;
}
