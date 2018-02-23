use std::io;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::multiplex::ServerProto;
use tokio_timer::Timer;

use monitor::TrafficCounting;
use protocol::{BrpcProtocol, HttpProtocol, ProtoCodec, Protocol, RpcProtocol};
use message::{RequestPackage, ResponsePackage};

use super::connection::TcpConnection;
use super::Second;

#[derive(Debug)]
pub struct MetaServerProtocol {
    protocols: Vec<Box<RpcProtocol>>,
    timer: Timer,
    idle_secs: Second,
    finished: Arc<AtomicUsize>,
}

impl MetaServerProtocol {
    pub fn new(
        protocols: Vec<Protocol>,
        timer: Timer,
        idle_secs: Second,
        finished: Arc<AtomicUsize>,
    ) -> Self {
        let protocols: Vec<_> = protocols
            .iter()
            .map(|proto| match proto {
                &Protocol::Brpc => Box::new(BrpcProtocol::new()) as Box<RpcProtocol>,
                &Protocol::Http => Box::new(HttpProtocol::new()) as Box<RpcProtocol>,
            })
            .collect();

        MetaServerProtocol {
            protocols,
            timer,
            idle_secs,
            finished,
        }
    }
}

impl<T> ServerProto<T> for MetaServerProtocol
where
    T: AsyncRead + AsyncWrite + 'static,
{
    type Request = RequestPackage;
    type Response = ResponsePackage;
    type Transport = TrafficCounting<Framed<TcpConnection<T>, ProtoCodec>>;
    type BindTransport = io::Result<Self::Transport>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        trace!("New connection established");
        let connection = TcpConnection::new(io, self.timer.clone(), self.idle_secs);
        let codec = ProtoCodec::new(self.protocols.as_slice());
        let transport = TrafficCounting::new(self.finished.clone(), connection.framed(codec));

        Ok(transport)
    }
}
