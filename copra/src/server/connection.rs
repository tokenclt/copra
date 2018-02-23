use bytes::{Buf, BufMut};
use futures::{Async, Poll};
use std::io::{self, Read, Write};
use std::mem;
use std::time::Duration;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_timer::{Sleep, Timer};

use super::Second;

pub struct TcpConnection<T> {
    io: T,
    timer: Timer,
    idle_timeout: Sleep,
    idle_secs: Second,
}

impl<T> TcpConnection<T> {
    pub fn new(io: T, timer: Timer, idle: Second) -> Self {
        let init_timeout = timer.sleep(Duration::from_secs(idle));
        TcpConnection {
            io,
            timer,
            idle_timeout: init_timeout,
            idle_secs: idle,
        }
    }
}

impl<T: Read> Read for TcpConnection<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.io.read(buf)
    }
}

impl<T: Write> Write for TcpConnection<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.io.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.io.flush()
    }
}

impl<T: AsyncRead> AsyncRead for TcpConnection<T> {
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        self.io.prepare_uninitialized_buffer(buf)
    }

    fn read_buf<B: BufMut>(&mut self, buf: &mut B) -> Poll<usize, io::Error> {
        // signal EOF
        if self.idle_timeout.is_expired() {
            // TODO: log IP
            trace!("Server closed a connection due to idle timeout");
            return Ok(Async::Ready(0));
        }

        let read = try_ready!(self.io.read_buf(buf));
        // reset timeout
        let new_timer = self.timer.sleep(Duration::from_secs(self.idle_secs));
        let _ = mem::replace(&mut self.idle_timeout, new_timer);

        Ok(Async::Ready(read))
    }
}

impl<T: AsyncWrite> AsyncWrite for TcpConnection<T> {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        <AsyncWrite>::shutdown(&mut self.io)
    }

    fn write_buf<B: Buf>(&mut self, buf: &mut B) -> Poll<usize, io::Error> {
        self.io.write_buf(buf)
    }
}
