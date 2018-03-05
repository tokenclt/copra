//! Find method by service name and method name

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use service::{BoxedNewUnifiedMethod, BoxedUnifiedMethod};

/// Dispatcher can not find the RPC method
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DispatchError {
    /// RPC service not found
    ServiceNotFound(String),
    /// RPC method not found
    MethodNotFound(String, String),
}

impl fmt::Display for DispatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::DispatchError::*;
        match *self {
            ServiceNotFound(ref n) => write!(f, "service {} not found", n),
            MethodNotFound(ref s, ref m) => write!(f, "method {} not found in {}", m, s),
        }
    }
}

impl Error for DispatchError {
    fn description(&self) -> &str {
        use self::DispatchError::*;
        match *self {
            ServiceNotFound(..) => "service not found",
            MethodNotFound(..) => "method not found",
        }
    }
}

/// Manage service registration and request dispatch
///
/// This struct is required to build a server. Service providers should add
/// their services to this struct.
pub struct ServiceRegistry {
    registry: HashMap<String, HashMap<String, BoxedNewUnifiedMethod>>,
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
        self.registry
            .insert(<T as NamedRegistrant>::name().to_string(), map);
    }

    /// Get a method by service name and method name.
    ///
    /// This method is used internally by generated stubs.
    pub fn get_method(
        &self,
        service_name: String,
        method_name: String,
    ) -> Result<BoxedUnifiedMethod, DispatchError> {
        self.registry
            .get(&service_name)
            .ok_or(DispatchError::ServiceNotFound(service_name.clone()))
            .and_then(|methods| {
                methods
                    .get(&method_name)
                    .ok_or(DispatchError::MethodNotFound(service_name, method_name))
                    .map(|s| s.new_service().unwrap())
            })
    }
}

/// Link method names with methods
///
/// This trait is automatically implemented by code generator. You do not
/// need to touch it.
pub trait Registrant {
    /// Get a list of name-method pairs.
    fn methods(&self) -> Vec<(String, BoxedNewUnifiedMethod)>;
}

/// Link service name to a registrant
///
/// This trait is automatically implemented by code generator. You do not
/// need to touch it.
pub trait NamedRegistrant: Registrant {
    /// Get service name.
    fn name() -> &'static str;
}
