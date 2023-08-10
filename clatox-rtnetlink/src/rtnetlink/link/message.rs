use libc::*;

use crate::netlink::{Attribute, Payload, RawAttributeIter, RouteType, Type};
use crate::utils::{align_attribute_len, read_u16, read_u32};

use super::{ArpHardware, InterfaceFlags, InterfaceInfoAttribute};

/// The message on which the `GetLink`, `AddLink`, `DeleteLink` requests are based
/// on. It corresponds to the `ifinfomsg` struct in libc.
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
        let mut buffer = Vec::with_capacity(16);
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

        let family = read_u16(iter.by_ref().cloned()).unwrap() & 0xFF;
        let device_type = read_u16(iter.by_ref().cloned()).unwrap();
        let index = read_u32(iter.by_ref().cloned()).unwrap() as i32;
        let flags = read_u32(iter.by_ref().cloned()).unwrap();
        let change = read_u32(iter.by_ref().cloned()).unwrap();

        let device_type = ArpHardware::from_raw_value(device_type)?;
        let flags = InterfaceFlags::from_bits(flags)?;
        // We have read 16 bytes so far. Align it to NLA_ALIGNTO bytes and start
        // deserializing the attributes.
        let aligned_len = align_attribute_len(16);
        for _ in 0..(aligned_len - 16) {
            iter.next()?;
        }

        let attributes = RawAttributeIter::new(iter.cloned())
            .map(|a| {
                let typ = a.attr_type();
                let attr = InterfaceInfoAttribute::from_raw(a);
                println!("{}, {:?}", typ, attr);
                attr
            })
            .try_collect()?;

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

/// A message that is of the `RTM_NEWLINK` type.
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

/// A message that is of the `RTM_GETLINK` type.
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

/// A message that is of the `RTM_DELLINK` type.
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
