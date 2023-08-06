use super::{Flags, Type};

/// The header of a Netlink message. It is equivalent to a Netlink message
/// without a payload.
pub type Header = Message<()>;

/// This struct corresponds to `nlmsghdr` in libc, along with the payload.
/// 
/// To calculate the length of a Message, use `serialize()` and measure the
/// length of the resulting byte array. If you are sure that the Message
/// contains only normal, plain-old-data structs (which is not the case for
/// e.g. Route messsages), you can measure the length using `mem::size_of()`.
/// 
/// If you need to obtain the length as specified in the header, deserialize
/// your message into a `Message<()>` and call `length()` on it.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message<T: Sized> {
    length: u32,
    message_type: Type,
    flags: Flags,
    seq: u32,
    pid: u32,
    payload: T,
}

/// Payload in a Netlink message.
pub trait Payload: Sized {
    fn message_type() -> Type;

    fn serialize(&self) -> Box<[u8]>;

    fn deserialize(bytes: &[u8]) -> Option<Self>;
}

impl Payload for () {
    fn message_type() -> Type {
        Type::Noop
    }

    fn serialize(&self) -> Box<[u8]> {
        Box::new([])
    }

    fn deserialize(_bytes: &[u8]) -> Option<Self> {
        Some(())
    }
}

impl Payload for Message<()> {
    fn message_type() -> Type {
        Type::Error
    }

    fn serialize(&self) -> Box<[u8]> {
        self.serialize()
    }

    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Self::deserialize(bytes)
    }
} 

impl Message<()> {
    /// Gets the length as specified in the header of a message.
    ///
    /// **NOTE:** This will only work for received messages! 
    pub fn length(&self) -> u32 {
        self.length
    }
}

impl<T: Payload> Message<T> {
    /// Create a new message containing given flags and payload.
    pub fn new(flags: Flags, payload: T) -> Self {
        Self {
            length: 16,
            message_type: T::message_type(),
            flags: flags,
            seq: 0,
            pid: 0,
            payload,
        }
    }

    pub fn serialize(&self) -> Box<[u8]> {
        let mut buffer = Vec::with_capacity(16);

        buffer.extend(self.length.to_ne_bytes().into_iter());
        buffer.extend(self.message_type.raw_value().to_ne_bytes().into_iter());
        buffer.extend(self.flags.bits().to_ne_bytes().into_iter());
        buffer.extend(self.seq.to_ne_bytes().into_iter());
        buffer.extend(self.pid.to_ne_bytes().into_iter());
        buffer.extend(self.payload.serialize().into_iter());

        let length = buffer.len() as u32;

        for (i, byte) in length.to_ne_bytes().into_iter().enumerate() {
            buffer[i] = byte;
        }

        buffer.into_boxed_slice()
    }

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        fn read_u16<I: Iterator<Item = u8>>(mut iter: I) -> Option<u16> {
            let bytes = [iter.next()?, iter.next()?];

            Some(u16::from_ne_bytes(bytes))
        }

        fn read_u32<I: Iterator<Item = u8>>(mut iter: I) -> Option<u32> {
            let bytes = [iter.next()?, iter.next()?, iter.next()?, iter.next()?];

            Some(u32::from_ne_bytes(bytes))
        }

        // The header is 16 bytes. If the data we receive is shorter than that
        // it's not going to be valid
        if bytes.len() < 16 {
            return None;
        }

        let mut iter = bytes.iter();
        let header_only = std::mem::size_of::<T>() == 0;

        let length = read_u32(iter.by_ref().take(4).cloned()).unwrap();
        let message_type = read_u16(iter.by_ref().take(2).cloned()).unwrap();

        let flags = read_u16(iter.by_ref().take(2).cloned()).unwrap();
        let seq = read_u32(iter.by_ref().take(4).cloned()).unwrap();
        let pid = read_u32(iter.by_ref().take(4).cloned()).unwrap();

        let message_type = Type::from_raw_value(message_type)?;
        let flags = unsafe { Flags::with_bits(flags) };

        // The `()` "payload" means that we get only the headers.
        if !header_only && message_type != T::message_type() {
            return None
        }

        let the_rest = iter.as_slice();
        let payload = T::deserialize(&the_rest)?;

        Some(Message {
            length: length,
            message_type,
            flags,
            seq,
            pid,
            payload,
        })
    }

    pub const fn message_type(&self) -> Type {
        self.message_type
    }

    pub const fn flags(&self) -> Flags {
        // SAFETY: the flags in self.flags are guaranteed to be valid Netlink flags
        self.flags
    }

    pub const fn seq(&self) -> u32 {
        self.seq
    }

    pub const fn pid(&self) -> u32 {
        self.pid
    }

    pub const fn payload(&self) -> &T {
        &self.payload
    }

    pub fn into_payload(self) -> T {
        self.payload
    }
}
