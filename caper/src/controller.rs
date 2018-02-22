//! [WIP] Service controller

use std::collections::HashMap;

use protocol::http::HttpStatus;

/// Expose more message details to service provider, and help to process
/// http requests.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Controller {
    /// Request target in http message
    pub http_url: Option<String>,
    /// Http headers
    pub headers: HashMap<String, String>,
    /// Http status code 
    pub status: Option<HttpStatus>,
    /// Request body in raw bytes
    pub request_body: Vec<u8>,
    /// Response body in raw bytes
    pub response_body: Vec<u8>,
}

impl Controller {
    /// Set Content-Type field in http headers
    pub fn set_content_type(&mut self, s: &str) {
        self.headers
            .insert("Content-Type".to_string(), s.to_string());
    }
}
