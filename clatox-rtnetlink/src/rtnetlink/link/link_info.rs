use libc::*;

use crate::attribute::RawAttributeIter;
use crate::utils::{self, align_attribute_len};

/// Information of a link interface. Corresponds to `IFLA_INFO_*` in libc.
///
/// LinkInfo is really similar to InterfaceInfoAttribute in format. In Netlink
/// terms, it is called a nested attribute.
///
#[doc(alias("ifinfomsg", "IFLA_INFO_"))]
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum LinkInfo {
    /// `IFLA_INFO_UNSPEC`
    Unspecified(Vec<u8>),
    /// `IFLA_INFO_KIND`
    Kind(Vec<u8>),
    /// `IFLA_INFO_DATA`
    Data(Vec<u8>),
    /// `IFLA_INFO_XSTATS`
    ExtendedStats(Vec<u8>),
    /// `IFLA_INFO_SLAVE_KIND`
    SlaveKind(Vec<u8>),
    /// `IFLA_INFO_SLAVE_DATA`
    SlaveData(Vec<u8>),
}

impl LinkInfo {
    pub fn deserialize(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut iter = RawAttributeIter::new(bytes.iter().cloned());
        let attr = iter.next()?;

        let attr_len = align_attribute_len(attr.length() as i32) as usize;
        let attr_type = attr.attr_type();
        let content = attr.into_payload();

        let attr = match attr_type {
            IFLA_INFO_UNSPEC => LinkInfo::Unspecified(content),
            IFLA_INFO_KIND => LinkInfo::Kind(content),
            IFLA_INFO_DATA => LinkInfo::Data(content),
            IFLA_INFO_XSTATS => LinkInfo::ExtendedStats(content),
            IFLA_INFO_SLAVE_KIND => LinkInfo::SlaveKind(content),
            IFLA_INFO_SLAVE_DATA => LinkInfo::SlaveData(content),
            x @ _ => panic!("unknown LinkInfo type: {}", x),
        };

        Some((attr, attr_len))
    }

    pub fn serialize_into(&self, buffer: &mut Vec<u8>) {
        utils::serialize_attribute_into(buffer, |buffer| match self {
            Self::Unspecified(content) => {
                buffer.extend(content.iter());
                IFLA_INFO_UNSPEC
            }
            Self::Kind(content) => {
                buffer.extend(content.iter());
                IFLA_INFO_KIND
            }
            Self::Data(content) => {
                buffer.extend(content.iter());
                IFLA_INFO_DATA
            }
            Self::ExtendedStats(content) => {
                buffer.extend(content.iter());
                IFLA_INFO_XSTATS
            }
            Self::SlaveKind(content) => {
                buffer.extend(content.iter());
                IFLA_INFO_SLAVE_KIND
            }
            Self::SlaveData(content) => {
                buffer.extend(content.iter());
                IFLA_INFO_SLAVE_DATA
            }
        });
    }
}
