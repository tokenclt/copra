use futures::{Future, Poll};

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

    fn call(&self, req: Self::Request) -> Self::Future;
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

    type SendRequest: SendRequest<
        Request = Self::Request,
        Response = Self::Response,
        Error = Self::Error,
    >;

    /// Get a end port (SendRequest).
    fn end_port(&self) -> Self::SendRequest;

    /// Perform a tick.
    fn poll(&mut self) -> Poll<(), Self::TickError>;
}
