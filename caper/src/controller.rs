use httparse::Header;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Controller {
    pub http_url: Option<String>,
    pub headers: HashMap<String, String>,
    pub request_body: Vec<u8>,
    pub response_body: Vec<u8>,
}