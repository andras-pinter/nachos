use std::io::{Read, Write};
use std::os::unix::io::{FromRawFd, RawFd};
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::Sink;
use crate::error::SocketError;


pub struct Socket {
    guac_socket: libguac::guac_socket,
    client_socket: SocketStream,
}

impl Socket {
    pub fn new() -> Result<Self, SocketError> {
        let socket = internal::SocketPair::new()?;

        let guac_socket = libguac::guac_socket::open(socket.parent)
            .ok_or(SocketError::UnableToOpen)?;
        guac_socket.require_keep_alive();

        let client_socket = unsafe {
            SocketStream::from_raw_fd(socket.child)
        };

        Ok(Socket {
            guac_socket,
            client_socket,
        })
    }

    pub fn socket(&self) -> Result<SocketStream, SocketError> {
        self.client_socket.try_clone().map_err(|_| SocketError::UnableToClone)
    }
}

impl AsRef<libguac::guac_socket> for Socket {
    fn as_ref(&self) -> &libguac::guac_socket {
        &self.guac_socket
    }
}

pub struct SocketStream(socketpair::SocketpairStream);

impl SocketStream {
    pub fn read(&mut self, buffer: &mut Vec<u8>) -> std::io::Result<usize> {
        buffer.clear();
        let mut buff = [0u8; 8192];
        println!("reading..");
        let len = self.0.read(&mut buff)?;
        println!("finished reading..");
        println!("buffer: {}", String::from_utf8_lossy(&buff));
        buffer.extend_from_slice(&buff[..len]);

        Ok(len)
    }

    pub fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.0.write(buffer)
    }

    fn try_clone(&self) -> std::io::Result<Self> {
        Ok(SocketStream(self.0.try_clone()?))
    }
}

impl FromRawFd for SocketStream {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        SocketStream(socketpair::SocketpairStream::from_raw_fd(fd))
    }
}

impl<I> Sink<I> for SocketStream
where Vec<u8>: From<I>
{
    type Error = SocketError;

    fn poll_ready(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(mut self: Pin<&mut Self>, item: I) -> Result<(), Self::Error> {
        let mut buffer = Vec::from(item);
        self.0.write(&mut buffer)
            .map(|_| ())
            .map_err(SocketError::SendError)
    }

    fn poll_flush(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(self.0.flush().map_err(SocketError::SendError))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

mod internal {
    use std::os::unix::io::RawFd;
    use super::SocketError;

    pub struct SocketPair {
        pub parent: RawFd,
        pub child: RawFd,
    }

    impl SocketPair {
        pub fn new() -> Result<Self, SocketError> {
            let (parent, child) = nix::sys::socket::socketpair(
                nix::sys::socket::AddressFamily::Unix,
                nix::sys::socket::SockType::Datagram,
                None,
                nix::sys::socket::SockFlag::empty(),
            )?;

            Ok(SocketPair {
                parent,
                child
            })
        }
    }
}
