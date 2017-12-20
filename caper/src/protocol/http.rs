use bytes::{BufMut, Bytes, BytesMut};
use std::io;
use std::str;
use std::collections::HashMap;
use tokio_proto::multiplex::RequestId;
use httparse::Request as HttpRequest;
use httparse::Status;
use httparse;

use controller::Controller;
use super::{ProtocolError, RpcProtocol};
use message::{RpcMeta, RpcRequestMeta};

#[derive(Clone, PartialEq, Debug)]
pub enum HttpStatus {
    Ok,
    Forbidden,
}

impl HttpStatus {
    pub fn to_code(&self) -> i32 {
        match *self {
            HttpStatus::Ok => 200,
            HttpStatus::Forbidden => 403,
        }
    }

    pub fn to_status_line(&self) -> &'static str {
        match *self {
            HttpStatus::Ok => "200 OK",
            HttpStatus::Forbidden => "403 Forbidden",
        }
    }
}

#[derive(Clone)]
enum HttpParseState {
    ReadingHeader,
    /// (header length, content length, controller)
    ReadingContent(usize, usize, RpcMeta, Controller),
}

#[derive(Clone)]
pub struct HttpProtocol {
    state: HttpParseState,
}

impl HttpProtocol {
    pub fn new() -> Self {
        HttpProtocol {
            state: HttpParseState::ReadingHeader,
        }
    }

    fn parse_name(&self, path: &str) -> Result<(String, String), ProtocolError> {
        let names: Vec<_> = path.split("/").filter(|s| s.len() > 0).collect();

        if names.len() < 2 {
            debug!("Http request: can not parse method name from path {}", path);
            Err(ProtocolError::AbsolutelyWrong)
        } else {
            Ok((names[0].to_string(), names[1].to_string()))
        }
    }
}

impl RpcProtocol for HttpProtocol {
    fn try_parse(
        &mut self,
        buf: &mut BytesMut,
    ) -> Result<(RequestId, (RpcMeta, Controller, Bytes)), ProtocolError> {
        loop {
            match self.state {
                HttpParseState::ReadingHeader => {
                    let mut headers = [httparse::EMPTY_HEADER; 10];
                    let mut req = HttpRequest::new(&mut headers);
                    match req.parse(buf) {
                        Ok(Status::Complete(header_len)) => {
                            debug!("Parsed a http header with the length of {}", header_len);
                            let mut controller = Controller::default();
                            let mut header_map = HashMap::new();
                            let mut request_meta = RpcRequestMeta::new();
                            let mut meta = RpcMeta::new();

                            //TODO: Verify http method

                            for header in req.headers {
                                let val = str::from_utf8(header.value).map_err(|_| {
                                    debug!("Http request: header error, field {} contains invalied utf-8 sequence", header.name);
                                    ProtocolError::AbsolutelyWrong
                                })?;
                                header_map.insert(header.name.to_string(), val.to_string());
                            }

                            let path = req.path.ok_or_else(|| {
                                debug!("Http request: request does not have path");
                                ProtocolError::AbsolutelyWrong
                            })?;

                            let id = header_map.get("Correlation-Id").map_or_else(
                                || {
                                    debug!("Http request header does not contain Correlation-Id, using default 0");
                                    Ok(0)
                                },
                                |s| {
                                    s.parse().map_err(|_| {
                                        debug!("Http request: invalid Correlation-Id");
                                        ProtocolError::AbsolutelyWrong
                                    })
                                },
                            )?;

                            let content_len = match header_map.get("Content-Length") {
                                Some(s) => s.parse().map_err(|_| {
                                    debug!("Http request: invalid value of Content-Length");
                                    ProtocolError::AbsolutelyWrong
                                })?,
                                None => 0,
                            };

                            debug!("Http request path: {}", path);
                            controller.http_url = Some(path.to_string());
                            controller.headers = header_map;

                            let (service, method) = self.parse_name(path)?;
                            request_meta.set_service_name(service);
                            request_meta.set_method_name(method);

                            meta.set_request(request_meta);
                            meta.set_correlation_id(id);

                            self.state = HttpParseState::ReadingContent(
                                header_len,
                                content_len,
                                meta,
                                controller,
                            );
                        }
                        Ok(Status::Partial) => return Err(ProtocolError::NeedMoreBytes),
                        Err(e) => {
                            debug!("Http header parse error: {:?}", e);
                            return Err(ProtocolError::AbsolutelyWrong);
                        }
                    }
                }
                HttpParseState::ReadingContent(header_len, content_len, ..) => {
                    if buf.len() < (header_len + content_len) {
                        return Err(ProtocolError::NeedMoreBytes);
                    }
                    debug!("Http request: begin to parse request body");
                    let state = ::std::mem::replace(&mut self.state, HttpParseState::ReadingHeader);

                    buf.split_to(header_len);

                    if let HttpParseState::ReadingContent(.., meta, mut controller) = state {
                        let body = buf.split_to(content_len as usize).freeze();
                        controller.request_body = Vec::from(body.as_ref());
                        self.state = HttpParseState::ReadingHeader;
                        debug!(
                            "Http request: Parsed a package with the length of {}",
                            header_len + content_len
                        );
                        return Ok((meta.get_correlation_id(), (meta, controller, Bytes::new())));
                    } else {
                        unreachable!();
                    }
                }
            }
        }
    }

    fn new_boxed(&self) -> Box<RpcProtocol> {
        Box::new(HttpProtocol {
            state: HttpParseState::ReadingHeader,
        })
    }

    fn write_package(
        &self,
        meta: (RpcMeta, Controller, Bytes),
        buf: &mut BytesMut,
    ) -> io::Result<()> {
        let (_meta, controller, _) = meta;
        let status = controller.status.unwrap_or(HttpStatus::Ok);
        let status_line = format!("HTTP/1.1 {}\r\n", status.to_status_line());
        match status {
            HttpStatus::Ok => {
                let mut headers = controller.headers;

                let content_len = controller.response_body.len();
                headers
                    .entry("Content-Length".to_string())
                    .or_insert(content_len.to_string());
                let header_len: usize = headers
                    .iter()
                    .map(|(key, val)| key.as_bytes().len() + val.as_bytes().len() + 4)
                    .sum();
                let response_len = status_line.as_bytes().len() + header_len + 2 + content_len;

                let free_len = buf.remaining_mut();
                debug!("Free {}, required {}", free_len, response_len);
                if free_len < response_len {
                    buf.reserve(response_len);
                }

                buf.put_slice(status_line.as_bytes());
                for (key, val) in headers.iter() {
                    buf.put_slice(key.as_bytes());
                    buf.put_slice(b": ");
                    buf.put_slice(val.as_bytes());
                    buf.put_slice(b"\r\n");
                }
                buf.put_slice(b"\r\n");
                buf.put_slice(&controller.response_body);
            }
            _ => {
                let response_len = status_line.as_bytes().len();
                let free_len = buf.remaining_mut();
                if free_len < response_len {
                    buf.reserve(response_len);
                }

                buf.put_slice(status_line.as_bytes());
            }
        }
        Ok(())
    }
}
