use bytes::{Bytes, BytesMut};
use futures::{Async, AsyncSink, Poll, Sink, StartSend, Stream};
use h2::{self, RecvStream, SendStream};
use http::Response;

use message::RpcResponseMeta;

#[derive(Debug)]
pub enum CodecError {
    Http2Error(h2::Error),
    ResponseHeaderError,
}

impl From<h2::Error> for CodecError {
    fn from(e: h2::Error) -> Self {
        CodecError::Http2Error(e)
    }
}

#[derive(Debug)]
pub struct ReadMessage {
    inner: RecvStream,
    partial: Option<BytesMut>,
}

impl ReadMessage {
    pub fn new(inner: RecvStream) -> Self {
        ReadMessage {
            inner,
            partial: None,
        }
    }
}

impl Stream for ReadMessage {
    type Item = Bytes;
    type Error = CodecError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct WriteMessage {
    inner: SendStream<Bytes>,
    buf: Option<Bytes>,
}

impl WriteMessage {
    pub fn new(inner: SendStream<Bytes>) -> Self {
        WriteMessage { inner, buf: None }
    }
}

impl Sink for WriteMessage {
    type SinkItem = Bytes;
    type SinkError = CodecError;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        unimplemented!()
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        unimplemented!()
    }
}

pub fn decode_meta(
    resp: Response<RecvStream>,
) -> Result<(RpcResponseMeta, RecvStream), CodecError> {
    unimplemented!()
}
