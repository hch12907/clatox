use libc::*;

use crate::attribute::RawAttributeIter;
use crate::utils::{self, align_attribute_len};

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
    Other(u8, Vec<u8>),
}

impl AddressFamilySpecific {
    pub fn deserialize(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut iter = RawAttributeIter::new(bytes.iter().cloned());
        let attr = iter.next()?;

        let attr_len = align_attribute_len(attr.length() as i32) as usize;
        let attr_type = attr.attr_type();
        let content = attr.into_payload();

        let attr = match attr_type as i32 {
            AF_INET => AddressFamilySpecific::Inet(content),
            AF_INET6 => AddressFamilySpecific::Inet6(content),
            typ => AddressFamilySpecific::Other(typ as u8, content),
        };

        Some((attr, attr_len))
    }

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
