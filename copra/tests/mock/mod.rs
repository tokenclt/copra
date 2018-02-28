use bytes::Bytes;
use copra::protocol::{BrpcProtocol, ProtoCodec};
use copra::message::ResponsePackage;
use futures::{Future, Sink, Stream};
use std::io::{self, Write};
use std::time::Duration;
use std::mem::replace;
use std::net::Shutdown;
use tokio_core::reactor::Handle;
use tokio_core::net::{Incoming, TcpListener, TcpStream};
use tokio_io::AsyncRead;
use tokio_io::codec::Framed;
use tokio_proto::multiplex::RequestId;
use tokio_timer::Timer;

pub enum Operation {
    Package(Box<Fn() -> ResponsePackage + Send>, Duration),
    Bytes(Box<Fn(RequestId) -> Bytes + Send>, Duration),
    Shutdown,
}

pub struct MockServerBuilder {
    listener: TcpListener,
    operations: Vec<Operation>,
}

impl MockServerBuilder {
    pub fn new<'a>(addr: &'a str, handle: Handle) -> Self {
        let socket_addr = addr.parse().expect("invalid address");
        let listener = TcpListener::bind(&socket_addr, &handle).expect("failed to bind address");

        MockServerBuilder {
            listener,
            operations: vec![],
        }
    }

    pub fn build(self) -> MockServer {
        MockServer::new(self.operations, self.listener)
    }

    pub fn respond_package<F>(&mut self, f: F, delay: Duration)
    where
        F: Fn() -> ResponsePackage + Send + 'static,
    {
        self.operations.push(Operation::Package(Box::new(f), delay));
    }

    pub fn respond_bytes<F>(&mut self, f: F, delay: Duration)
    where
        F: Fn(RequestId) -> Bytes + Send + 'static,
    {
        self.operations.push(Operation::Bytes(Box::new(f), delay));
    }

    pub fn close_connection(&mut self) {
        self.operations.push(Operation::Shutdown);
    }

    // how to mimic connection broke ?
    // consider how client retries

    // lessons learned: carefully exam the expected behaivors before developping
}

enum ServerState {
    HasConnection(Framed<TcpStream, ProtoCodec>, Incoming),
    Listening(Incoming),
    Temp,
}

pub struct MockServer {
    operations: Vec<Operation>,
    state: ServerState,
    timer: Timer,
}

impl MockServer {
    pub fn new(ops: Vec<Operation>, listener: TcpListener) -> Self {
        MockServer {
            operations: ops,
            state: ServerState::Listening(listener.incoming()),
            timer: Timer::default(),
        }
    }

    pub fn start(&mut self) -> Result<(), io::Error> {
        loop {
            if self.operations.is_empty() {
                return Ok(());
            }

            match replace(&mut self.state, ServerState::Temp) {
                ServerState::Listening(listener) => {
                    let (next, listener) = listener.into_future().wait().map_err(|(e, _)| e)?;
                    let (socket, _) = match next {
                        Some(s) => s,
                        None => return Ok(()),
                    };
                    let stream = socket.framed(ProtoCodec::new(&[Box::new(BrpcProtocol::new())]));
                    self.state = ServerState::HasConnection(stream, listener);
                }
                ServerState::HasConnection(stream, listener) => {
                    let (next, mut stream) = stream.into_future().wait().map_err(|(e, _)| e)?;
                    let (id, _req) = match next {
                        Some(s) => s,
                        None => return Ok(()),
                    };

                    match self.operations.pop().unwrap() {
                        Operation::Package(f, delay) => {
                            self.timer.sleep(delay).wait().unwrap();
                            stream = stream.send((id, f())).wait()?;
                            self.state = ServerState::HasConnection(stream, listener);
                        }
                        Operation::Bytes(f, delay) => {
                            self.timer.sleep(delay).wait().unwrap();
                            self.wait_send(stream.get_mut(), f(id))?;
                            self.state = ServerState::HasConnection(stream, listener);
                        }
                        Operation::Shutdown => {
                            stream.into_inner().shutdown(Shutdown::Both)?;
                            self.state = ServerState::Listening(listener);
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    fn wait_send<T: Write>(&self, io: &mut T, mut bytes: Bytes) -> Result<(), io::Error> {
        loop {
            let wrote = io.write(&bytes)?;
            io.flush()?;
            if wrote == bytes.len() {
                return Ok(());
            }
            bytes.split_to(wrote);
        }
    }
}
