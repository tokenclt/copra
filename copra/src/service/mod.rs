//! Internal types helping to provide services

use bytes::Bytes;
use futures::{Future, IntoFuture};
use std::fmt;
use std::io;
use std::sync::Arc;
use tokio_service::NewService;

use controller::Controller;
use codec::MethodCodec;
use dispatcher::{ServiceRegistry};
use message::{RequestPackage, ResponsePackage, RpcResponseMeta};

pub use tokio_service::Service;
pub use self::error::{ProviderSetError, RequestProcessError};

mod error;

type Bundle = (Bytes, Controller);

pub(crate) type MetaServiceFuture = Box<Future<Item = ResponsePackage, Error = io::Error>>;

/// A future that will resolve to a serialized message
pub type MethodFuture = Box<Future<Item = Bundle, Error = RequestProcessError>>;

/// Type alias of `Service` trait object
///
/// This type is used internally by auto-generated stubs.
#[doc(hidden)]
pub type BoxedUnifiedMethod = Box<
    Service<
        Request = Bundle,
        Response = Bundle,
        Error = RequestProcessError,
        Future = MethodFuture,
    >
        + Send
        + Sync,
>;

/// Type alias of `NewService` trait object
///
/// This type is used internally by auto-generated stubs.
#[doc(hidden)]
pub type BoxedNewUnifiedMethod = Box<
    NewService<
        Request = Bundle,
        Response = Bundle,
        Error = RequestProcessError,
        Instance = BoxedUnifiedMethod,
    >
        + Send
        + Sync,
>;

#[derive(Clone, Debug)]
pub(crate) struct MetaService {
    registry: Arc<ServiceRegistry>,
}

impl MetaService {
    pub fn new(registry: Arc<ServiceRegistry>) -> Self {
        MetaService { registry }
    }
}

impl Service for MetaService {
    type Request = RequestPackage;
    type Response = ResponsePackage;
    type Error = io::Error;
    type Future = MetaServiceFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        let (meta, controller, body) = req;
        // find method in the registry by service name and method name
        let method = {
            let service_name = meta.service_name;
            let method_name = meta.method_name;
            self.registry
                .get_method(service_name, method_name)
                // TODO: insert log here
                .map_err(|e| RequestProcessError::from(e))
        };
        let resp = method
            .into_future()
            .and_then(|method| method.call((body, controller)))
            // Convert error type to error code, and fill it in the response
            // meta
            .then(|resp| {
                let pkg = match resp {
                    Ok((body, ctrl)) => {
                        let mut meta = RpcResponseMeta::new();
                        // zero means no error
                        meta.set_error_code(0);
                        (meta, ctrl, body)
                    }
                    Err(e) => {
                        let mut meta = RpcResponseMeta::new();
                        meta.set_error_code(e.error_code());
                        meta.set_error_text(format!("{}", e));
                        // Bytes::new() will not allocate
                        let empty = Bytes::new();
                        (meta, Controller::default(), empty)
                    }
                };
                Ok(pkg)
            });

        // TODO: unbox this
        Box::new(resp)
    }
}

impl NewService for MetaService {
    type Request = RequestPackage;
    type Response = ResponsePackage;
    type Error = io::Error;
    type Instance = Self;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

/// A factory struct that can produce encapsulated service.
///
/// An encapsulated service consists of the method codec and the user-defined
/// processing logic.
pub struct NewUnifiedMethod<S: Send + Sync> {
    inner: Box<
        NewService<Request = Bundle, Response = Bundle, Error = RequestProcessError, Instance = S>
            + Send
            + Sync,
    >,
}

impl<S> fmt::Debug for NewUnifiedMethod<S>
where
    S: Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NewEncapsulatedMethod")
    }
}

impl<S> NewUnifiedMethod<S>
where
    S: NewService<Request = Bundle, Response = Bundle, Error = RequestProcessError, Instance = S>,
    S: Service<
        Request = Bundle,
        Response = Bundle,
        Error = RequestProcessError,
        Future = MethodFuture,
    >,
    S: 'static + Send + Sync,
{
    /// Create a new instance by boxing.
    pub fn new(method: S) -> Self {
        NewUnifiedMethod {
            inner: Box::new(method),
        }
    }
}

impl<S> NewService for NewUnifiedMethod<S>
where
    S: NewService<Request = Bundle, Response = Bundle, Error = RequestProcessError, Instance = S>,
    S: Service<
        Request = Bundle,
        Response = Bundle,
        Error = RequestProcessError,
        Future = MethodFuture,
    >,
    S: 'static + Send + Sync,
{
    type Request = Bundle;
    type Response = Bundle;
    type Error = RequestProcessError;
    type Instance = BoxedUnifiedMethod;

    fn new_service(&self) -> io::Result<Self::Instance> {
        self.inner
            .new_service()
            .map(|s| Box::new(s) as BoxedUnifiedMethod)
    }
}

/// A bunble of a codec and a user-defined service
///
/// This struct is used to provide a unified interface for the request dispatcher.
#[allow(missing_debug_implementations)]
pub struct CodecMethodBundle<C, S> {
    codec: C,
    method: S,
}

impl<C, S> CodecMethodBundle<C, S> where {
    /// Create a new bundle from a codec and a service
    pub fn new(codec: C, method: S) -> Self {
        CodecMethodBundle {
            codec: codec,
            method: method,
        }
    }
}

impl<C, S> Service for CodecMethodBundle<C, S>
where
    C: MethodCodec + Clone + 'static,
    S: Service<
        Request = (C::Request, Controller),
        Response = (C::Response, Controller),
        Error = ProviderSetError,
    >,
    S: Clone + 'static,
{
    type Request = Bundle;
    type Response = Bundle;
    type Error = RequestProcessError;
    type Future = MethodFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        let codec = self.codec.clone();
        let method = self.method.clone();
        let (body, controller) = req;
        let fut = codec
            .decode(body)
            .map_err(|_| RequestProcessError::RequestDecodeError)
            .into_future()
            .and_then(move |body| {
                method
                    .call((body, controller))
                    .map_err(|e| RequestProcessError::ProviderSetError(e))
                    .and_then(move |(body, controller)| {
                        codec
                            .encode(body)
                            .map_err(|_| RequestProcessError::ResponseEncodeError)
                            .map(|body| (body, controller))
                    })
            });
        Box::new(fut)
    }
}

impl<C, S> NewService for CodecMethodBundle<C, S>
where
    C: MethodCodec + Clone + 'static,
    S: Service<
        Request = (C::Request, Controller),
        Response = (C::Response, Controller),
        Error = ProviderSetError,
    >,
    S: Clone + 'static,
{
    type Request = Bundle;
    type Response = Bundle;
    type Error = RequestProcessError;
    type Instance = Self;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(CodecMethodBundle {
            codec: self.codec.clone(),
            method: self.method.clone(),
        })
    }
}
