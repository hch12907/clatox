use libc::*;

use crate::netlink::{Payload, RouteType, Type};
use crate::rtnetlink::align_attribute_len;
use crate::utils::{read_u16, read_u32};

use super::{ArpHardware, InterfaceFlags, InterfaceInfoAttribute};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceInfoMessage {
    family: u16, // This is an unsigned char in ifinfomsg. Use u16 to cover the padding.
    device_type: ArpHardware,
    index: i32,
    flags: InterfaceFlags,
    change: u32,
    attributes: Vec<InterfaceInfoAttribute>,
}

impl InterfaceInfoMessage {
    pub const fn new(
        device_type: ArpHardware,
        index: i32,
        flags: InterfaceFlags,
        attributes: Vec<InterfaceInfoAttribute>,
    ) -> Self {
        Self {
            family: AF_UNSPEC as u16,
            device_type,
            index,
            flags,
            change: !0u32,
            attributes,
        }
    }

    pub const fn device_type(&self) -> ArpHardware {
        self.device_type
    }

    pub const fn index(&self) -> i32 {
        self.index
    }

    pub const fn flags(&self) -> InterfaceFlags {
        self.flags
    }

    pub fn attributes(&self) -> &[InterfaceInfoAttribute] {
        &self.attributes
    }

    pub fn serialize(&self) -> Box<[u8]> {
        let mut buffer = vec![0u8; 16];
        buffer.extend(self.family.to_ne_bytes().into_iter());
        buffer.extend(self.device_type.raw_value().to_ne_bytes().into_iter());
        buffer.extend(self.index.to_ne_bytes().into_iter());
        buffer.extend(self.flags.bits().to_ne_bytes().into_iter());
        buffer.extend(self.change.to_ne_bytes().into_iter());

        let aligned_len = (buffer.len() as i32 + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1);
        for _ in 0..(aligned_len - buffer.len() as i32) {
            buffer.push(0u8);
        }

        for attr in &self.attributes {
            buffer.extend(attr.serialize().iter())
        }

        buffer.into_boxed_slice()
    }

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        // The header is 16 bytes. If the data we receive is shorter than that
        // it's not going to be valid
        if bytes.len() < 16 {
            return None;
        }

        let mut iter = bytes.iter();

        let family = read_u16(iter.by_ref().take(2).cloned()).unwrap() & 0xFF;
        let device_type = read_u16(iter.by_ref().take(2).cloned()).unwrap();
        let index = read_u32(iter.by_ref().take(4).cloned()).unwrap() as i32;
        let flags = read_u32(iter.by_ref().take(4).cloned()).unwrap();
        let change = read_u32(iter.by_ref().take(4).cloned()).unwrap();

        let device_type = ArpHardware::from_raw_value(device_type)?;
        let flags = unsafe { InterfaceFlags::with_bits(flags) };

        // We have read 16 bytes so far. Align it to NLA_ALIGNTO bytes and start
        // deserializing the attributes.
        let aligned_len = align_attribute_len(16);
        for _ in 0..(aligned_len - 16) {
            iter.next()?;
        }

        let mut attributes = Vec::new();

        let mut remaining_len = bytes.len() - aligned_len as usize;
        while remaining_len > 2 {
            let bytes = iter.as_slice();
            let (attr, len) = match InterfaceInfoAttribute::deserialize(bytes) {
                Some(attr) => attr,
                None => break,
            };
            attributes.push(attr);

            iter.by_ref().nth(len - 1).unwrap();
            remaining_len -= len;
        }

        Some(InterfaceInfoMessage {
            family,
            device_type,
            index,
            flags,
            change,
            attributes,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewLink(pub InterfaceInfoMessage);

impl Payload for NewLink {
    fn message_type() -> Type {
        Type::Route(RouteType::NewLink)
    }

    fn serialize(&self) -> Box<[u8]> {
        self.0.serialize()
    }

    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(Self(InterfaceInfoMessage::deserialize(bytes)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetLink(pub InterfaceInfoMessage);

impl Payload for GetLink {
    fn message_type() -> Type {
        Type::Route(RouteType::GetLink)
    }

    fn serialize(&self) -> Box<[u8]> {
        self.0.serialize()
    }

    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(Self(InterfaceInfoMessage::deserialize(bytes)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteLink(pub InterfaceInfoMessage);

impl Payload for DeleteLink {
    fn message_type() -> Type {
        Type::Route(RouteType::DeleteLink)
    }

    fn serialize(&self) -> Box<[u8]> {
        self.0.serialize()
    }

    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(Self(InterfaceInfoMessage::deserialize(bytes)?))
    }
}
