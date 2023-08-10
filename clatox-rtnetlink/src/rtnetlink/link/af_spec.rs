use libc::*;

use crate::netlink::{Attribute, RawAttribute};
use crate::rtnetlink::AddressFamily;
use crate::utils;

/// Address-family specific information of a link interface.
///
/// Corresponds to `IFLA_AFSPEC` and `IFLA_INET` in libc.
///
/// TODO: Much of this remains to be typed.
#[doc(alias("ifinfomsg", "IFLA_AFSPEC"))]
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum AddressFamilySpecific {
    /// `IFLA_INET_*`
    Inet(Vec<u8>),

    /// `IFLA_INET6_*`
    Inet6(Vec<u8>),

    /// Unrecognized address families
    Other(AddressFamily, Vec<u8>),
}

impl AddressFamilySpecific {
    pub fn serialize_in(&self, buffer: &mut Vec<u8>) {
        utils::serialize_attribute_into(buffer, |buffer| match self {
            Self::Inet(content) => {
                buffer.extend(content.iter());
                AF_INET as u16
            }
            Self::Inet6(content) => {
                buffer.extend(content.iter());
                AF_INET6 as u16
            }
            Self::Other(typ, content) => {
                buffer.extend(content.iter());
                *typ as u16
            }
        });
    }
}

impl Attribute for AddressFamilySpecific {
    fn from_raw(raw: RawAttribute) -> Option<Self> {
        let attr_type = raw.attr_type();
        let content = raw.into_payload();

        let attr = match attr_type as i32 {
            AF_INET => Self::Inet(content),
            AF_INET6 => Self::Inet6(content),
            typ => Self::Other(AddressFamily::from_raw_value(typ as u8)?, content),
        };

        Some(attr)
    }
}
