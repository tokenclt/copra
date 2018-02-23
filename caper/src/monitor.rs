//! [WIP] Built-in service for monitoring the server

use futures::{Async, AsyncSink, Poll, Sink, StartSend, Stream};
use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, SystemTime};
use tokio_proto::multiplex::RequestId;
use tokio_timer::{Interval, Timer};

use message::{RequestPackage, ResponsePackage};

/// A transport middleware that counts the processed mssages
#[derive(Debug)]
pub struct TrafficCounting<T> {
    buffered: Arc<AtomicUsize>,
    flushed: Arc<AtomicUsize>,
    io: T,
}

impl<T> TrafficCounting<T> {
    /// Create a new transport middleware on top of `io`, store the number in
    /// `flushed`.
    pub fn new(flushed: Arc<AtomicUsize>, io: T) -> Self {
        TrafficCounting {
            buffered: Arc::new(AtomicUsize::new(0)),
            flushed,
            io,
        }
    }
}

impl<T> Stream for TrafficCounting<T>
where
    T: Stream<Item = (RequestId, RequestPackage), Error = io::Error>,
{
    type Item = (RequestId, RequestPackage);

    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.io.poll()
    }
}

impl<T> Sink for TrafficCounting<T>
where
    T: Sink<SinkItem = (RequestId, ResponsePackage), SinkError = io::Error>,
{
    type SinkItem = (RequestId, ResponsePackage);

    type SinkError = io::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        let res = self.io.start_send(item)?;
        if let AsyncSink::Ready = res {
            self.buffered.fetch_add(1, Ordering::Relaxed);
        }
        Ok(res)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        let res = self.io.poll_complete()?;
        if let Async::Ready(_) = res {
            let buffered = self.buffered.swap(0, Ordering::SeqCst);
            self.flushed.fetch_add(buffered, Ordering::SeqCst);
        }
        Ok(res)
    }
}

/// A maintainer calculates throughput periodically
///
/// This maintainer implements `Stream` so that it can be spawned
/// as a daemon
#[derive(Debug)]
pub struct ThroughputMaintainer {
    interval: Interval,
    finished: Arc<AtomicUsize>,
    throughput: Arc<AtomicUsize>,
    last_fired: SystemTime,
}

impl ThroughputMaintainer {
    /// Create a new maintainer, export throughput value to `throughtput`
    pub fn new(timer: Timer, finished: Arc<AtomicUsize>, throughput: Arc<AtomicUsize>) -> Self {
        let interval = timer.interval(Duration::from_secs(1));
        ThroughputMaintainer {
            interval,
            finished,
            throughput,
            last_fired: SystemTime::now(),
        }
    }
}

impl Stream for ThroughputMaintainer {
    type Item = ();

    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        try_ready!(self.interval.poll().map_err(|_| ()));

        let new_time = SystemTime::now();
        let elapse = new_time
            .duration_since(self.last_fired)
            .map_err(|_| error!("SystemTime compare error."))?;
        self.last_fired = new_time;

        let finished = self.finished.swap(0, Ordering::SeqCst);
        let elapse = elapse.as_secs() as f32 + (elapse.subsec_nanos() as f32 / 1e9);
        let throughput = (finished as f32 / elapse) as usize;
        info!("New finished: {}, throughput {}", finished, throughput);
        self.throughput.store(throughput, Ordering::SeqCst);

        Ok(Async::Ready(Some(())))
    }
}
