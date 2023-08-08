use libc::*;

use crate::attribute::{Attribute, RawAttribute};
use crate::utils;

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

impl Attribute for LinkInfo {
    fn from_raw(raw: RawAttribute) -> Option<Self> {
        let attr_type = raw.attr_type();
        let content = raw.into_payload();

        let attr = match attr_type {
            IFLA_INFO_UNSPEC => LinkInfo::Unspecified(content),
            IFLA_INFO_KIND => LinkInfo::Kind(content),
            IFLA_INFO_DATA => LinkInfo::Data(content),
            IFLA_INFO_XSTATS => LinkInfo::ExtendedStats(content),
            IFLA_INFO_SLAVE_KIND => LinkInfo::SlaveKind(content),
            IFLA_INFO_SLAVE_DATA => LinkInfo::SlaveData(content),
            x @ _ => panic!("unknown LinkInfo type: {}", x),
        };

        Some(attr)
    }
}
