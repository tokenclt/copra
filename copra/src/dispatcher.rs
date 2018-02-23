//! Find method by service name and method name

use std::collections::HashMap;
use std::fmt;

use service::{EncapService, NewEncapService};

/// Manage service registration and request dispatch
///
/// This struct is required to build a server. Service providers should add
/// their services to this struct.
pub struct ServiceRegistry {
    registry: HashMap<String, HashMap<String, NewEncapService>>,
}

impl fmt::Debug for ServiceRegistry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(
                self.registry
                    .iter()
                    .map(|(k, v)| (k, v.keys().collect::<Vec<_>>())),
            )
            .finish()
    }
}

impl ServiceRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        ServiceRegistry {
            registry: HashMap::new(),
        }
    }

    /// Add a new service to the registry.
    pub fn register_service<T>(&mut self, registrant: T)
    where
        T: NamedRegistrant,
    {
        let mut map = HashMap::new();
        for (method_name, encap) in registrant.methods().into_iter() {
            map.insert(method_name, encap);
        }
        self.registry.insert(<T as NamedRegistrant>::name().to_string(), map);
    }

    /// Get a method by service name and method name.
    /// 
    /// This method is used internally by generated stubs.
    pub fn get_method(&self, service_name: &str, method_name: &str) -> Option<EncapService> {
        self.registry
            .get(service_name)
            .and_then(|methods| methods.get(method_name))
            .map(|s| s.new_service().unwrap())
    }
}


/// Link method names with methods
/// 
/// This trait is automatically implemented by code generator. You do not
/// need to touch it.
pub trait Registrant {
    /// Get a list of name-method pairs.
    fn methods(&self) -> Vec<(String, NewEncapService)>;
}

/// Link service name to a registrant
/// 
/// This trait is automatically implemented by code generator. You do not
/// need to touch it.
pub trait NamedRegistrant: Registrant {
    /// Get service name.
    fn name() -> &'static str;
}