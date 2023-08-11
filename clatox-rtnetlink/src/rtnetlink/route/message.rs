use crate::netlink::{Attribute, Payload, RawAttributeIter, RouteType as RouteMessageType, Type};
use crate::rtnetlink::AddressFamily;
use crate::utils::{align_attribute_len, read_u32};

use super::{RouteAttribute, RouteFlags, RouteProtocol, RouteScope, RouteTable, RouteType};

#[doc(alias("rtmsg"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteMessage {
    family: AddressFamily,
    dst_len: u8,
    src_len: u8,
    tos: u8,
    table: RouteTable,
    protocol: RouteProtocol,
    scope: RouteScope,
    route_type: RouteType,
    flags: RouteFlags,
    attributes: Vec<RouteAttribute>,
}

impl RouteMessage {
    pub const fn new(
        family: AddressFamily,
        dst_len: u8,
        src_len: u8,
        tos: u8,
        table: RouteTable,
        protocol: RouteProtocol,
        scope: RouteScope,
        route_type: RouteType,
        flags: RouteFlags,
        attributes: Vec<RouteAttribute>,
    ) -> Self {
        Self {
            family,
            dst_len,
            src_len,
            tos,
            table,
            protocol,
            scope,
            route_type,
            flags,
            attributes,
        }
    }

    pub const fn family(&self) -> AddressFamily {
        self.family
    }

    pub const fn dst_len(&self) -> u8 {
        self.dst_len
    }

    pub const fn src_len(&self) -> u8 {
        self.src_len
    }

    pub const fn tos(&self) -> u8 {
        self.tos
    }

    pub const fn table(&self) -> &RouteTable {
        &self.table
    }

    pub const fn protocol(&self) -> &RouteProtocol {
        &self.protocol
    }

    pub const fn scope(&self) -> &RouteScope {
        &self.scope
    }

    pub const fn route_type(&self) -> &RouteType {
        &self.route_type
    }

    pub const fn flags(&self) -> RouteFlags {
        RouteFlags::from_bits_retain(self.flags.bits())
    }

    pub fn attributes(&self) -> &[RouteAttribute] {
        &self.attributes
    }

    pub fn serialize(&self) -> Box<[u8]> {
        let mut buffer = Vec::with_capacity(12);
        buffer.push(self.family.raw_value());
        buffer.push(self.dst_len);
        buffer.push(self.src_len);
        buffer.push(self.tos);
        buffer.push(self.table.raw_value());
        buffer.push(self.protocol.raw_value());
        buffer.push(self.scope.raw_value());
        buffer.push(self.route_type.raw_value());
        buffer.extend(self.flags.bits().to_ne_bytes().into_iter());

        let aligned_len = align_attribute_len(buffer.len() as i32);
        for _ in 0..(aligned_len - buffer.len() as i32) {
            buffer.push(0u8);
        }

        for attr in &self.attributes {
            attr.serialize_into(&mut buffer);
        }

        buffer.into_boxed_slice()
    }

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        // The header is 12 bytes. If the data we receive is shorter than that
        // it's not going to be valid
        if bytes.len() < 12 {
            return None;
        }

        let mut iter = bytes.iter().cloned();

        let family = iter.next()?;
        let dst_len = iter.next()?;
        let src_len = iter.next()?;
        let tos = iter.next()?;
        let table = iter.next()?;
        let protocol = iter.next()?;
        let scope = iter.next()?;
        let route_type = iter.next()?;
        let flags = read_u32(iter.by_ref())?;

        let family = AddressFamily::from_raw_value(family)?;
        let table = RouteTable::from_raw_value(table)?;
        let protocol = RouteProtocol::from_raw_value(protocol)?;
        let scope = RouteScope::from_raw_value(scope)?;
        let route_type = RouteType::from_raw_value(route_type)?;
        let flags = RouteFlags::from_bits(flags)?;

        let aligned_len = align_attribute_len(12);
        for _ in 0..(aligned_len - 12) {
            iter.next()?;
        }

        let attributes = RawAttributeIter::new(iter)
            .map(RouteAttribute::from_raw)
            .try_collect()?;

        Some(RouteMessage {
            family,
            dst_len,
            src_len,
            table,
            protocol,
            scope,
            route_type,
            flags,
            attributes,
            tos,
        })
    }
}

/// A message that is of the `RTM_NEWROUTE` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewRoute(pub RouteMessage);

impl Payload for NewRoute {
    fn message_type() -> Type {
        Type::Route(RouteMessageType::NewRoute)
    }

    fn serialize(&self) -> Box<[u8]> {
        self.0.serialize()
    }

    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(Self(RouteMessage::deserialize(bytes)?))
    }
}

/// A message that is of the `RTM_GETROUTE` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetRoute(pub RouteMessage);

impl Payload for GetRoute {
    fn message_type() -> Type {
        Type::Route(RouteMessageType::GetRoute)
    }

    fn serialize(&self) -> Box<[u8]> {
        self.0.serialize()
    }

    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(Self(RouteMessage::deserialize(bytes)?))
    }
}

/// A message that is of the `RTM_DELLINK` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteRoute(pub RouteMessage);

impl Payload for DeleteRoute {
    fn message_type() -> Type {
        Type::Route(RouteMessageType::DeleteRoute)
    }

    fn serialize(&self) -> Box<[u8]> {
        self.0.serialize()
    }

    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(Self(RouteMessage::deserialize(bytes)?))
    }
}
