use libc::*;
use socket2::{Domain, Protocol as RawProtocol, SockAddr, Socket as RawSocket, Type};

use std::io::{ErrorKind as IoErrorKind, IoSlice, IoSliceMut, Read, Result as IoResult, Write};
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};

use super::{Message, Payload, Protocol, Flags, types, ErrorMessage};

/// This corresponds to an opened socket which is bound to a `SocketAddr`.
/// Right now, `SocketAddr` is not implemented and `Socket` is hardcoded to
/// bind to `(pid: 0, groups: 0)`.
#[derive(Debug)]
pub struct Socket {
    socket: RawSocket,
}

/// Received message(s) or error from the socket.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceivedMessage<T> {
    /// A Netlink error message was received. The original Netlink header is
    /// provided to give user applications a clue which message was erroneous.
    Error(Message<ErrorMessage>),

    /// A Netlink message was successfully received.
    Message(Message<T>),

    /// Multiple fragmented (multipart) Netlink messages were received.
    /// It is generally not expected for multipart messages to be reassembled
    /// together in Netlink. The exact behaviour that is expected upon receiving
    /// multipart messages depends on the Netlink protocol that is in use. Some
    /// protocols expect them to be treated like separate messages.
    Multipart(Vec<Message<T>>),
}

impl Socket {
    pub fn connect_to_kernel(protocol: Protocol) -> IoResult<Socket> {
        let socket = RawSocket::new(
            Domain::from(AF_NETLINK),
            Type::from(SOCK_RAW),
            RawProtocol::from(protocol.raw_value()).into(),
        )?;

        // SAFETY: It is okay to fill sockaddr_storage with zeroes*. Furthermore,
        //         sockaddr_storage guarantees that it has the same/greater alignment
        //         and size than its subtypes (e.g. sockaddr_nl), thus ensuring that
        //         the casting is safe.
        //       * Except sa_family. But we fill it in immediately after the initial
        //         zeroing.
        let sockaddr = unsafe {
            let mut raw_sockaddr = std::mem::zeroed::<sockaddr_storage>();

            let sockaddr_nl_mut = &mut *((&mut raw_sockaddr as *mut _) as *mut sockaddr_nl);

            sockaddr_nl_mut.nl_family = AF_NETLINK as sa_family_t;
            sockaddr_nl_mut.nl_pid = 0; // destination is the kernel
            sockaddr_nl_mut.nl_groups = 0;

            SockAddr::new(
                raw_sockaddr,
                std::mem::size_of::<sockaddr_nl>() as socklen_t,
            )
        };

        socket.bind(&sockaddr)?;
        socket.set_send_buffer_size(1 << 16)?;
        socket.set_recv_buffer_size(1 << 18)?; // 256KB ought to be enough!! or not...

        Ok(Self { socket })
    }

    pub fn receive_message<T>(&mut self) -> IoResult<ReceivedMessage<T>>
    where
        T: Payload + std::fmt::Debug,
    {
        let recv_buffer_size = self.socket.recv_buffer_size()?;
        let mut buffer = vec![0u8; recv_buffer_size];
        let size = self.socket.read(buffer.as_mut_slice())?;

        if size == 0 {
            return Err(IoErrorKind::UnexpectedEof)?;
        }

        // SAFETY: The recv() call will never fill in more bytes than what the
        // buffer itself can hold. Hence the set_len will always be a shrinking
        // operation, and no uninitialized bytes will be read.
        unsafe {
            debug_assert!(size <= buffer.len());
            buffer.set_len(size);
        }

        let first_header = match Message::<()>::deserialize(&buffer[..size]) {
            Some(header) => header,
            None => Err(IoErrorKind::InvalidInput)?,
        };
        let is_multipart = first_header.flags().contains(Flags::Multi);
        let is_error = first_header.message_type() == types::Type::Error;

        if is_multipart {
            let mut messages = Vec::new();
            let mut header = first_header;

            while header.message_type() != types::Type::Done {
                let mut last_used_buffer = 0;
                let mut used_buffer = 0;

                while used_buffer < buffer.len() {
                    used_buffer += header.length() as usize;

                    let current_buf = &buffer[last_used_buffer..used_buffer];
                    let next_buf = &buffer[used_buffer..];

                    let message = match Message::<T>::deserialize(current_buf) {
                        Some(msg) => msg,
                        None => Err(IoErrorKind::InvalidInput)?,
                    };

                    messages.push(message);

                    header = match Message::<()>::deserialize(next_buf) {
                        Some(header) => header,
                        None if used_buffer < buffer.len() => Err(IoErrorKind::InvalidInput)?,
                        None => break,
                    };

                    last_used_buffer = used_buffer;
                }

                // SAFETY: The buffer is extended to its original capacity, and recvmsg()
                // writes to it, and we shrink it back to the initialized bytes immediately.
                unsafe {
                    buffer.set_len(recv_buffer_size);
                    let size = self.socket.read(buffer.as_mut_slice())?;
                    buffer.set_len(size);

                    header = match Message::<()>::deserialize(&buffer) {
                        Some(header) => header,
                        None => Err(IoErrorKind::InvalidInput)?,
                    };
                }
            }

            Ok(ReceivedMessage::Multipart(messages))
        } else if is_error {
            match Message::<ErrorMessage>::deserialize(&buffer[..size]) {
                Some(msg) => Ok(ReceivedMessage::Error(msg)),
                None => Err(IoErrorKind::InvalidInput)?,
            }
        } else {
            match Message::<T>::deserialize(&buffer[..size]) {
                Some(msg) => Ok(ReceivedMessage::Message(msg)),
                None => Err(IoErrorKind::InvalidInput)?,
            }
        }
    }

    pub fn send_message<T>(&mut self, msg: &Message<T>) -> IoResult<usize>
    where
        T: Payload,
    {
        self.socket.write(msg.serialize().as_ref())
    }
}

impl AsFd for Socket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.socket.as_fd()
    }
}

impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> RawFd {
        self.socket.as_raw_fd()
    }
}

impl FromRawFd for Socket {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self {
            socket: RawSocket::from_raw_fd(fd),
        }
    }
}

impl IntoRawFd for Socket {
    fn into_raw_fd(self) -> RawFd {
        self.socket.into_raw_fd()
    }
}

impl Read for Socket {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.socket.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> IoResult<usize> {
        self.socket.read_vectored(bufs)
    }
}

impl Write for Socket {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.socket.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> IoResult<usize> {
        self.socket.write_vectored(bufs)
    }

    fn flush(&mut self) -> IoResult<()> {
        self.socket.flush()
    }
}
