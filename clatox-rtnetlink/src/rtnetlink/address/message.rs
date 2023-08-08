use libc::*;

use crate::netlink::{Payload, Type, RouteType};
use crate::rtnetlink::AddressFamily;
use crate::utils::{align_attribute_len, read_u32};
use crate::{Attribute, RawAttributeIter};

use super::{AddressFlags, InterfaceAddressAttribute};

/// The message on which the `GetAddress`, `AddAddress`, `DeletAddress` requests
/// are based on. It corresponds to the `ifaddrmsg` struct in libc.
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc(alias("ifaddrmsg"))]
pub struct InterfaceAddressMessage {
    family: AddressFamily,
    prefixlen: u8,
    flags: AddressFlags,
    scope: u8,
    index: u32,
    attributes: Vec<InterfaceAddressAttribute>,
}

impl InterfaceAddressMessage {
    pub const fn new(
        family: AddressFamily,
        prefixlen: u8,
        flags: AddressFlags,
        attributes: Vec<InterfaceAddressAttribute>,
    ) -> Self {
        Self {
            family: family,
            prefixlen,
            flags: flags,
            scope: 0,
            index: 0,
            attributes,
        }
    }

    pub const fn family(&self) -> AddressFamily {
        self.family
    }

    pub const fn index(&self) -> u32 {
        self.index
    }

    pub const fn flags(&self) -> AddressFlags {
        self.flags
    }

    pub fn attributes(&self) -> &[InterfaceAddressAttribute] {
        &self.attributes
    }

    pub fn serialize(&self) -> Box<[u8]> {
        let mut buffer = Vec::with_capacity(16);
        buffer.push(self.family.raw_value());
        buffer.push(self.prefixlen);
        buffer.push(self.flags.bits());
        buffer.push(self.scope);
        buffer.extend(self.index.to_ne_bytes().into_iter());

        let aligned_len = (buffer.len() as i32 + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1);
        for _ in 0..(aligned_len - buffer.len() as i32) {
            buffer.push(0u8);
        }

        for attr in &self.attributes {
            attr.serialize_into(&mut buffer)
        }

        buffer.into_boxed_slice()
    }

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        // The header is 16 bytes. If the data we receive is shorter than that
        // it's not going to be valid
        if bytes.len() < 8 {
            return None;
        }

        let mut iter = bytes.iter().cloned();

        let family = iter.next()?;
        let prefixlen = iter.next()?;
        let flags = iter.next()?;
        let scope = iter.next()?;
        let index = read_u32(iter.by_ref()).unwrap();

        let family = AddressFamily::from_raw_value(family)?;
        let flags = unsafe { AddressFlags::with_bits(flags) };

        // We have read 8 bytes so far. Align it to NLA_ALIGNTO bytes and start
        // deserializing the attributes.
        let aligned_len = align_attribute_len(8);
        for _ in 0..(aligned_len - 8) {
            iter.next()?;
        }

        let attributes = RawAttributeIter::new(iter)
            .map(InterfaceAddressAttribute::from_raw)
            .try_collect()?;

        Some(InterfaceAddressMessage {
            family,
            prefixlen,
            flags,
            scope,
            index,
            attributes,
        })
    }
}

/// A message that is of the `RTM_GETADDR` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetAddress(pub InterfaceAddressMessage);

impl Payload for GetAddress {
    fn message_type() -> Type {
        Type::Route(RouteType::GetAddress)
    }

    fn serialize(&self) -> Box<[u8]> {
        self.0.serialize()
    }

    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(Self(InterfaceAddressMessage::deserialize(bytes)?))
    }
}

/// A message that is of the `RTM_NEWADDR` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewAddress(pub InterfaceAddressMessage);

impl Payload for NewAddress {
    fn message_type() -> Type {
        Type::Route(RouteType::NewAddress)
    }

    fn serialize(&self) -> Box<[u8]> {
        self.0.serialize()
    }

    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(Self(InterfaceAddressMessage::deserialize(bytes)?))
    }
}

/// A message that is of the `RTM_DELADDR` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteAddress(pub InterfaceAddressMessage);

impl Payload for DeleteAddress {
    fn message_type() -> Type {
        Type::Route(RouteType::DeleteAddress)
    }

    fn serialize(&self) -> Box<[u8]> {
        self.0.serialize()
    }

    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(Self(InterfaceAddressMessage::deserialize(bytes)?))
    }
}
