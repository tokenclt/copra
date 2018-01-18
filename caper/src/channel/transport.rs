use futures::{AsyncSink, Poll, Sink, Stream, StartSend};
use std::io;
use tokio_proto::multiplex::RequestId;

use super::{RequestPackage, ResponsePackage};

pub struct Transport<T> {
    io: T,
}

impl<T> Transport<T> {
    pub fn new(io: T) -> Self {
        Transport { io }
    }
}

impl<T> Stream for Transport<T>
where
    T: Stream<Item = (RequestId, ResponsePackage), Error = io::Error>,
{
    type Item = (RequestId, ResponsePackage);
    
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.io.poll()
    }
}

impl<T> Sink for Transport<T>
where
    T: Sink<SinkItem = (RequestId, RequestPackage), SinkError = io::Error>,
{
    type SinkItem = (RequestId, RequestPackage);

    type SinkError = io::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.io.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.io.poll_complete()
    }
}