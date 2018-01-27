use futures::{Async, Future, Poll, Stream};
use futures::stream::FuturesUnordered;
use futures::sync::oneshot;
use tokio_core::reactor::Handle;
use tokio_service::Service;

use super::{ChannelReceiver, OneShotSender, RequestPackage, ResponsePackage};
use load_balancer::LoadBalance;

use super::{FeedbackHandle, FeedbackReceiver};

#[must_use = "Channel backend must be spawned in a reactor, otherwise no request will be sent"]
pub struct ChannelBackend {
    handle: Handle,
    lb: Box<LoadBalance>,
    recv: ChannelReceiver,
    feedbacks: FuturesUnordered<FeedbackReceiver>,
}

impl ChannelBackend {
    pub fn new<L>(recv: ChannelReceiver, handle: Handle, lb: L) -> Self
    where
        L: LoadBalance + 'static,
    {
        ChannelBackend {
            recv,
            handle,
            lb: Box::new(lb) as Box<LoadBalance>,
            feedbacks: FuturesUnordered::new(),
        }
    }

    fn spawn(&mut self, resp_sender: OneShotSender, req: RequestPackage) {
        trace!("Spawned a new rpc request.");

        let (server_id, end_port) = self.lb.select_server();
        let (fb_sender, fb_recv) = oneshot::channel();
        let fut = end_port.call(req).then(move |result| {
            let fb_handle = FeedbackHandle::new(server_id, fb_sender);
            // TODO: Or maybe just ignore this error, for the rpc request might be cancelled.
            resp_sender
                .send(result.map(move |r| (r, fb_handle)))
                .expect("The receiving end of the oneshot is dropped.");

            Ok(())
        });

        self.feedbacks.push(fb_recv);
        self.handle.spawn(fut);
    }
}

impl Future for ChannelBackend {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            // check returned feedback info
            // TODO: error handling?
            //       the oneshot channel might be broken (due to dropped sender).
            if let Ok(Async::Ready(Some((server_id, call_info)))) = self.feedbacks.poll() {
                self.lb.feed_back(server_id, call_info);
            }
            // spawn new request
            match try_ready!(self.recv.poll()) {
                Some((resp_sender, req)) => self.spawn(resp_sender, req),
                None => return Ok(Async::Ready(())),
            }
        }
    }
}
