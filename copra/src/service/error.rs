use std::error::Error;
use std::fmt;

use dispatcher::DispatchError;

/// Error set by a service provider
///
/// An RPC service can return this error if it decides that the request
/// broke some contrains.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProviderSetError {
    /// The request message is invalid (i.e. business contrains violated )
    InvalidRequest,
    /// The request message is legal, but the server failed to precess the request
    InternalError,
}

impl fmt::Display for ProviderSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProviderSetError::InvalidRequest => write!(f, "the request message is invalid"),
            ProviderSetError::InternalError => write!(f, "internal server error"),
        }
    }
}

impl Error for ProviderSetError {
    fn description(&self) -> &str {
        match *self {
            ProviderSetError::InvalidRequest => "the request message is invalid",
            ProviderSetError::InternalError => "internal server error",
        }
    }
}

/// Error raised during processing a request
///
/// This error can be produced if the method codec fails to decode request message,
/// or if the service provider marks the RPC as failure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RequestProcessError {
    /// Service provider set RPC to failure
    ProviderSetError(ProviderSetError),
    /// Can not find requested method
    DispatchError(DispatchError),
    /// failed to decode request
    RequestDecodeError,
    /// failed to encode response
    ResponseEncodeError,
}

impl RequestProcessError {
    /// Get error code for the type of error
    ///
    /// The error code is used to transmit error information through
    /// RPC protocols. It is automatically handled by the servier and channel.
    pub fn error_code(&self) -> i32 {
        unimplemented!()
    }
}

impl fmt::Display for RequestProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RequestProcessError::*;
        match *self {
            ProviderSetError(ref e) => write!(f, "provider returned an error {}", e),
            DispatchError(ref e) => write!(f, "dispatch error: {}", e),
            RequestDecodeError => write!(f, "failed to decode request"),
            ResponseEncodeError => write!(f, "failed to encode response"),
        }
    }
}

impl Error for RequestProcessError {
    fn description(&self) -> &str {
        use self::RequestProcessError::*;
        match *self {
            ProviderSetError(_) => "provider returned an error",
            DispatchError(_) => "dispatch error",
            RequestDecodeError => "failed to decode request",
            ResponseEncodeError => "failed to encode response",
        }
    }

    fn cause(&self) -> Option<&Error> {
        use self::RequestProcessError::*;
        match *self {
            ProviderSetError(ref e) => Some(e),
            DispatchError(ref e) => Some(e),
            RequestDecodeError => None,
            ResponseEncodeError => None,
        }
    }
}

impl From<ProviderSetError> for RequestProcessError {
    fn from(e: ProviderSetError) -> Self {
        RequestProcessError::ProviderSetError(e)
    }
}

impl From<DispatchError> for RequestProcessError {
    fn from(e: DispatchError) -> Self {
        RequestProcessError::DispatchError(e)
    }
}
