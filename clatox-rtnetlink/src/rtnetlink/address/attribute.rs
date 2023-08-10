use std::mem::{size_of, transmute};

use libc::*;

use crate::netlink::{Attribute, RawAttribute};
use crate::utils;

use super::{AddressCacheInfo, AddressFlags};

pub const IFA_RT_PRIORITY: u16 = 9;
pub const IFA_TARGET_NETNSID: u16 = 10;
pub const IFA_PROTO: u16 = 11;

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc(alias("ifaddrmsg"))]
pub enum InterfaceAddressAttribute {
    /// `IFA_UNSPEC`
    Unspecified(Vec<u8>),

    /// `IFA_ADDRESS`
    Address(Vec<u8>),

    /// `IFA_LOCAL`
    Local(Vec<u8>),

    /// `IFA_LABEL`
    Label(String),

    /// `IFA_BROADCAST`
    Broadcast(Vec<u8>),

    /// `IFA_ANYCAST`
    Anycast(Vec<u8>),

    /// `IFA_CACHEINFO`
    CacheInfo(AddressCacheInfo),

    /// `IFA_MULTICAST`
    Multicast(Vec<u8>),

    /// `IFA_FLAGS`
    Flags(AddressFlags),

    /// `IFA_RT_PRIORITY`
    RoutePriority(u32),

    /// `IFA_TARGET_NETNSID`
    TargetNetNamespaceId(Vec<u8>),

    /// `IFA_PROTO`
    Protocol(u8),

    Other(u16, Vec<u8>),
}

impl InterfaceAddressAttribute {
    pub fn serialize_into(&self, buffer: &mut Vec<u8>) {
        utils::serialize_attribute_into(buffer, |buffer| match self {
            InterfaceAddressAttribute::Unspecified(content) => {
                buffer.extend(content.iter());
                IFA_UNSPEC
            }
            InterfaceAddressAttribute::Address(content) => {
                buffer.extend(content.iter());
                IFA_ADDRESS
            }
            InterfaceAddressAttribute::Local(content) => {
                buffer.extend(content.iter());
                IFA_LOCAL
            }
            InterfaceAddressAttribute::Label(label) => {
                buffer.extend(label.bytes());
                buffer.push(0u8); // zero-terminated string
                IFA_LABEL
            }
            InterfaceAddressAttribute::Broadcast(content) => {
                buffer.extend(content.iter());
                IFA_BROADCAST
            }
            InterfaceAddressAttribute::Anycast(content) => {
                buffer.extend(content.iter());
                IFA_ANYCAST
            }
            InterfaceAddressAttribute::CacheInfo(cache) => {
                // SAFETY: It is safe to transmute CacheInfo into a byte array as
                // the type does not contain any paddings.
                let bytes = unsafe {
                    transmute::<AddressCacheInfo, [u8; size_of::<AddressCacheInfo>()]>(
                        cache.clone(),
                    )
                };
                buffer.extend(bytes.iter().cloned());
                IFA_CACHEINFO
            }
            InterfaceAddressAttribute::Multicast(content) => {
                buffer.extend(content.iter());
                IFA_MULTICAST
            }
            InterfaceAddressAttribute::Flags(flag) => {
                buffer.extend(flag.bits().to_ne_bytes().iter());
                IFA_FLAGS
            }
            InterfaceAddressAttribute::RoutePriority(priority) => {
                buffer.extend(priority.to_ne_bytes().iter());
                IFA_RT_PRIORITY
            }
            InterfaceAddressAttribute::TargetNetNamespaceId(content) => {
                buffer.extend(content.iter());
                IFA_TARGET_NETNSID
            }
            InterfaceAddressAttribute::Protocol(prot) => {
                buffer.push(*prot);
                IFA_PROTO
            }
            InterfaceAddressAttribute::Other(typ, content) => {
                buffer.extend(content.iter());
                *typ
            }
        })
    }
}

impl Attribute for InterfaceAddressAttribute {
    fn from_raw(raw: RawAttribute) -> Option<Self> {
        let attr_type = raw.attr_type();
        let content = raw.into_payload();

        let attr = match attr_type {
            IFA_UNSPEC => InterfaceAddressAttribute::Unspecified(content),
            IFA_ADDRESS => InterfaceAddressAttribute::Address(content),
            IFA_LOCAL => InterfaceAddressAttribute::Local(content),
            IFA_LABEL => {
                let mut content = String::from_utf8(content).ok()?;
                let popped = content.pop();
                debug_assert!(popped == Some('\0'));
                InterfaceAddressAttribute::Label(content)
            }
            IFA_BROADCAST => InterfaceAddressAttribute::Broadcast(content),
            IFA_ANYCAST => InterfaceAddressAttribute::Anycast(content),
            IFA_CACHEINFO => {
                let content = <[u8; size_of::<AddressCacheInfo>()]>::try_from(content).ok()?;
                // SAFETY: AddressCacheInfo is a plain-old-data struct and contains no
                // paddings. We trust Rtnetlink to give us the correct values here, but
                // even if the values are wrong, there won't be a memory corruption or UB.
                let cache = unsafe {
                    transmute::<[u8; size_of::<AddressCacheInfo>()], AddressCacheInfo>(content)
                };
                InterfaceAddressAttribute::CacheInfo(cache)
            }
            IFA_MULTICAST => InterfaceAddressAttribute::Multicast(content),
            IFA_FLAGS => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                let flag = AddressFlags::from_bits_truncate(u32::from_ne_bytes(content));
                InterfaceAddressAttribute::Flags(flag)
            }
            IFA_RT_PRIORITY => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceAddressAttribute::RoutePriority(u32::from_ne_bytes(content))
            }
            IFA_TARGET_NETNSID => InterfaceAddressAttribute::TargetNetNamespaceId(content),
            IFA_PROTO => {
                let content = *content.get(0)?;
                InterfaceAddressAttribute::Protocol(content)
            }
            typ => InterfaceAddressAttribute::Other(typ, content),
        };

        Some(attr)
    }
}
