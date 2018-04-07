use futures::{Future, Poll};

pub mod http2;

/// Stub and middlewares use this to send request to backend
pub trait SendRequest {
    /// Type of request
    type Request;
    /// Type of response
    type Response;
    /// Error
    type Error;
    /// Future resolves to response
    type Future: Future<Item = Self::Response, Error = Self::Error>;
    /// Send a request
    fn call(&mut self, req: Self::Request) -> Self::Future;
    /// Polls the end port to see if it is ready to accept new request
    fn poll_ready(&mut self) -> Poll<(), Self::Error>;
}

/// Middleware and final backends. Stackable
pub trait Layer {
    /// Type of request
    type Request;
    /// Type of response
    type Response;
    /// Error
    type Error;
    /// Error for tick (poll)
    type TickError;
    /// Type of the end port
    type SendRequest: SendRequest<
        Request = Self::Request,
        Response = Self::Response,
        Error = Self::Error,
    >;

    /// Get a end port (SendRequest).
    fn end_port(&mut self) -> Self::SendRequest;

    /// Perform a tick.
    fn poll(&mut self) -> Poll<(), Self::TickError>;
}

/// Create new `Layer`s
///
/// Useful for reconnecting middlewares.
pub trait NewLayer {
    /// Type of request
    type Request;
    /// Type of response
    type Response;
    /// Type of request error
    type Error;
    /// Type of polling Error
    type TickError;
    /// Type of the end port
    type SendRequest: SendRequest<
        Request = Self::Request,
        Response = Self::Response,
        Error = Self::Error,
    >;
    /// Type of the created `Layer`
    type Layer: Layer<
        Request = Self::Request,
        Response = Self::Response,
        Error = Self::Error,
        TickError = Self::TickError,
        SendRequest = Self::SendRequest,
    >;
    /// Creation error
    type InitError;

    /// Init future
    type Future: Future<Item = Self::Layer, Error = Self::InitError>;

    /// Create a new instance of `Layer`
    fn new_layer(&self) -> Self::Future;
}
