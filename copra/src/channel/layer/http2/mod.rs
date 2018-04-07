use bytes::Bytes;
use channel::layer::{Layer, NewLayer, SendRequest};
use futures::{Async, Future, Poll, Sink, Stream};
use h2::client::Connection;
use h2::{self, client, RecvStream, SendStream};
use http::{Request, Response};
use message::{RpcRequestMeta, RpcResponseMeta};
use std::io;
use std::mem;
use std::net::SocketAddr;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;

use self::codec::{decode_meta, CodecError, ReadMessage, WriteMessage};

mod codec;

/// Error type of grpc channel
#[derive(Debug)]
pub enum GrpcError {
    /// Errors related to http2 communication
    Http2Error(h2::Error),
    // FIXME: inconsistent abstract. Move this error to another type
    /// Connection error
    ConnectError,
    /// Invalid response header
    ResponseHeaderError,
}

impl From<h2::Error> for GrpcError {
    fn from(e: h2::Error) -> Self {
        GrpcError::Http2Error(e)
    }
}

impl From<CodecError> for GrpcError {
    fn from(e: CodecError) -> Self {
        use self::CodecError::*;
        match e {
            Http2Error(e) => GrpcError::Http2Error(e),
            ResponseHeaderError => GrpcError::ResponseHeaderError,
        }
    }
}

#[derive(Debug)]
enum OnGoing {
    Sending(client::ResponseFuture, WriteMessage),
    RecvHead(client::ResponseFuture),
    RecvBody(ReadMessage, RpcResponseMeta),
    Null(GrpcError),
    Temp,
}

/// A future which will resolve to a unary(non-streaming) response
#[derive(Debug)]
pub struct ResponseFuture {
    state: OnGoing,
}

impl ResponseFuture {
    /// Create a new instance which will resolve to the response.
    pub fn new_ok(fut: client::ResponseFuture, send: SendStream<Bytes>, body: Bytes) -> Self {
        let mut write_msg = WriteMessage::new(send);
        // a newly created WriteMessage is always ready to buffer a message
        write_msg.start_send(body).unwrap();
        let state = OnGoing::Sending(fut, write_msg);
        ResponseFuture { state }
    }

    /// Create a new instance which will immidiately resolve to an error.
    pub fn new_err(e: GrpcError) -> Self {
        let state = OnGoing::Null(e);
        ResponseFuture { state }
    }
}

impl Future for ResponseFuture {
    type Item = (RpcResponseMeta, Bytes);

    type Error = GrpcError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::OnGoing::*;
        loop {
            match mem::replace(&mut self.state, Temp) {
                Sending(fut, mut send) => {
                    match send.poll_complete()? {
                        Async::NotReady => {
                            self.state = Sending(fut, send);
                            return Ok(Async::NotReady);
                        }
                        _ => {}
                    }
                    self.state = RecvHead(fut);
                }
                RecvHead(mut fut) => {
                    let resp = match fut.poll()? {
                        Async::NotReady => {
                            self.state = RecvHead(fut);
                            return Ok(Async::NotReady);
                        }
                        Async::Ready(resp) => resp,
                    };
                    let (meta, recv) = decode_meta(resp)?;
                    let read_msg = ReadMessage::new(recv);
                    self.state = RecvBody(read_msg, meta);
                }
                RecvBody(mut read, meta) => {
                    let body = match read.poll()? {
                        Async::NotReady => {
                            self.state = RecvBody(read, meta);
                            return Ok(Async::NotReady);
                        }
                        Async::Ready(Some(b)) => b,
                        // TODO: Unary None Maybe reachable?
                        _ => unreachable!(),
                    };

                    return Ok(Async::Ready((meta, body)));
                }
                Null(e) => return Err(e.into()),
                Temp => unreachable!(),
            }
        }
    }
}

/// An end port for sending request via grpc protocol
#[derive(Debug)]
pub struct GrpcSendRequest {
    inner: client::SendRequest<Bytes>,
}

impl GrpcSendRequest {
    /// Create a new instance.
    pub fn new(inner: client::SendRequest<Bytes>) -> Self {
        GrpcSendRequest { inner }
    }
}

impl SendRequest for GrpcSendRequest {
    type Request = (RpcRequestMeta, Bytes);
    type Response = (RpcResponseMeta, Bytes);
    type Error = GrpcError;
    type Future = ResponseFuture;

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let (meta, body) = req;
        let req = build_request(meta);

        match self.inner.send_request(req, false) {
            Ok((fut, send)) => ResponseFuture::new_ok(fut, send, body),
            Err(e) => ResponseFuture::new_err(e.into()),
        }
    }

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        //TODO: specify error
        self.inner.poll_ready().map_err(Into::into)
    }
}

/// Grpc layer serves as the base layer of client channel
#[derive(Debug)]
pub struct Grpc {
    connection: client::Connection<TcpStream, Bytes>,
    send: client::SendRequest<Bytes>,
}

impl Grpc {
    /// Create a new instance.
    pub fn new(send: client::SendRequest<Bytes>, connection: Connection<TcpStream, Bytes>) -> Self {
        Grpc { send, connection }
    }
}

impl Layer for Grpc {
    type Request = (RpcRequestMeta, Bytes);
    type Response = (RpcResponseMeta, Bytes);
    type Error = GrpcError;
    type TickError = GrpcError;
    type SendRequest = GrpcSendRequest;

    fn end_port(&mut self) -> Self::SendRequest {
        GrpcSendRequest::new(self.send.clone())
    }

    fn poll(&mut self) -> Poll<(), Self::TickError> {
        self.connection.poll().map_err(Into::into)
    }
}

/// Create new `Grpc` instance
#[derive(Debug)]
pub struct NewGrpc {
    addr: SocketAddr,
    handle: Handle,
}

impl NewLayer for NewGrpc {
    type Request = (RpcRequestMeta, Bytes);
    type Response = (RpcResponseMeta, Bytes);
    type Error = GrpcError;
    type TickError = GrpcError;
    type SendRequest = GrpcSendRequest;
    type Layer = Grpc;
    type InitError = GrpcError;
    type Future = Box<Future<Item = Self::Layer, Error = Self::InitError>>;

    fn new_layer(&self) -> Self::Future {
        let fut = TcpStream::connect(&self.addr, &self.handle)
            .map_err(|_| GrpcError::ConnectError)
            .and_then(|io| client::handshake(io).map_err(Into::into))
            .map(|(send, connection)| Grpc::new(send, connection));

        Box::new(fut)
    }
}

fn build_request(meta: RpcRequestMeta) -> Request<()> {
    unimplemented!()
}
