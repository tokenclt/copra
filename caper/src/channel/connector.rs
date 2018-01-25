use bytes::{Buf, BufMut};
use futures::{Async, Future, Poll};
use std::io::{self, ErrorKind, Read, Write};
use std::mem;
use std::net::SocketAddr;
use tokio_core::reactor::Handle;
use tokio_core::net::{TcpStream, TcpStreamNew};
use tokio_io::{AsyncRead, AsyncWrite};

enum State {
    Connected(TcpStream),
    Connecting(TcpStreamNew),
    Disconnected,
}

pub struct Connector {
    addr: SocketAddr,
    state: State,
    handle: Handle,
}

impl Connector {
    pub fn new(addr: SocketAddr, handle: Handle) -> Self {
        let new = TcpStream::connect(&addr, &handle);
        Connector {
            addr,
            state: State::Connecting(new),
            handle,
        }
    }

    pub fn from_stream(addr: SocketAddr, stream: TcpStream, handle: Handle) -> Self {
        Connector {
            addr,
            state: State::Connected(stream),
            handle,
        }
    }

    pub fn poll_ready(&mut self) -> Poll<(), io::Error> {
        match mem::replace(&mut self.state, State::Disconnected) {
            State::Connected(io) => {
                self.state = State::Connected(io);
                Ok(Async::Ready(()))
            }
            State::Connecting(mut fut) => match fut.poll()? {
                Async::Ready(io) => {
                    self.state = State::Connected(io);
                    Ok(Async::Ready(()))
                }
                Async::NotReady => {
                    self.state = State::Connecting(fut);
                    Ok(Async::NotReady)
                }
            },
            State::Disconnected => unreachable!(),
        }
    }

    fn reconnect(&mut self) {
        let new = TcpStream::connect(&self.addr, &self.handle);
        self.state = State::Connecting(new);
    }
}

impl Read for Connector {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            match mem::replace(&mut self.state, State::Disconnected) {
                State::Connected(mut io) => {
                    let r = io.read(buf);
                    if r.is_ok() {
                        self.state = State::Connected(io);
                    }
                    return r;
                }
                State::Connecting(mut fut) => match fut.poll()? {
                    Async::Ready(io) => {
                        self.state = State::Connected(io);
                    }
                    Async::NotReady => {
                        self.state = State::Connecting(fut);
                        return Err(io::Error::new(
                            ErrorKind::WouldBlock,
                            "Waiting for connection to be established",
                        ));
                    }
                },
                State::Disconnected => {
                    self.reconnect();
                }
            }
        }
    }
}

impl Write for Connector {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        loop {
            match mem::replace(&mut self.state, State::Disconnected) {
                State::Connected(mut io) => {
                    let r = io.write(buf);
                    if r.is_ok() {
                        self.state = State::Connected(io);
                    }
                    return r;
                }
                State::Connecting(mut fut) => match fut.poll()? {
                    Async::Ready(io) => {
                        self.state = State::Connected(io);
                    }
                    Async::NotReady => {
                        self.state = State::Connecting(fut);
                        return Err(io::Error::new(
                            ErrorKind::WouldBlock,
                            "Waiting for connection to be established",
                        ));
                    }
                },
                State::Disconnected => {
                    self.reconnect();
                }
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        loop {
            match mem::replace(&mut self.state, State::Disconnected) {
                State::Connected(mut io) => {
                    let r = io.flush();
                    if r.is_ok() {
                        self.state = State::Connected(io);
                    }
                    return r;
                }
                State::Connecting(mut fut) => match fut.poll()? {
                    Async::Ready(io) => {
                        self.state = State::Connected(io);
                    }
                    Async::NotReady => {
                        self.state = State::Connecting(fut);
                        return Err(io::Error::new(
                            ErrorKind::WouldBlock,
                            "Waiting for connection to be established",
                        ));
                    }
                },
                State::Disconnected => {
                    self.reconnect();
                }
            }
        }
    }
}

impl AsyncRead for Connector {
    // Ported from <TcpStream as AsyncRead>
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        false
    }

    fn read_buf<B: BufMut>(&mut self, buf: &mut B) -> Poll<usize, io::Error> {
        loop {
            match mem::replace(&mut self.state, State::Disconnected) {
                State::Connected(mut io) => {
                    let r = io.read_buf(buf);
                    if r.is_ok() {
                        self.state = State::Connected(io);
                    }
                    return r;
                }
                State::Connecting(mut fut) => match fut.poll()? {
                    Async::Ready(io) => {
                        self.state = State::Connected(io);
                    }
                    Async::NotReady => {
                        self.state = State::Connecting(fut);
                        return Ok(Async::NotReady);
                    }
                },
                State::Disconnected => {
                    self.reconnect();
                }
            }
        }
    }
}

impl AsyncWrite for Connector {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        match mem::replace(&mut self.state, State::Disconnected) {
            State::Connected(mut io) => {
                let r = <AsyncWrite>::shutdown(&mut io);
                self.state = State::Connected(io);
                r
            }
            State::Connecting(_) => {
                self.state = State::Disconnected;
                Ok(Async::Ready(()))
            }
            State::Disconnected => Ok(Async::Ready(())),
        }
    }

    fn write_buf<B: Buf>(&mut self, buf: &mut B) -> Poll<usize, io::Error> {
        loop {
            match mem::replace(&mut self.state, State::Disconnected) {
                State::Connected(mut io) => {
                    let r = io.write_buf(buf);
                    if r.is_ok() {
                        self.state = State::Connected(io);
                    }
                    return r;
                }
                State::Connecting(mut fut) => match fut.poll()? {
                    Async::Ready(io) => {
                        self.state = State::Connected(io);
                    }
                    Async::NotReady => {
                        self.state = State::Connecting(fut);
                        return Ok(Async::NotReady);
                    }
                },
                State::Disconnected => {
                    self.reconnect();
                }
            }
        }
    }
}

pub struct ConnectorInit {
    new: TcpStreamNew,
    other: Option<(SocketAddr, Handle)>,
}

impl Future for ConnectorInit {
    type Item = Connector;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let stream = try_ready!(self.new.poll());
        let (addr, handle) = mem::replace(&mut self.other, None)
            .expect("ConnectorInit can not be polled after returning Async::Ready");
        let conn = Connector::from_stream(addr, stream, handle);

        Ok(Async::Ready(conn))
    }
}
