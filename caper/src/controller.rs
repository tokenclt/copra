use std::collections::HashMap;

use protocol::http::HttpStatus;

#[derive(Default, Clone)]
pub struct Controller {
    pub http_url: Option<String>,
    pub headers: HashMap<String, String>,
    pub status: Option<HttpStatus>,
    pub request_body: Vec<u8>,
    pub response_body: Vec<u8>,
}

impl Controller {
    pub fn set_content_type(&mut self, s: &str) {
        self.headers
            .insert("Content-Type".to_string(), s.to_string());
    }
}
