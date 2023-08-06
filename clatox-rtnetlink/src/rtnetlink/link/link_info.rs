use libc::*;

use crate::utils::{align_attribute_len, read_u16};

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
        let mut iter = bytes.iter();

        let attr_len = read_u16(iter.by_ref().cloned()).unwrap();
        let attr_type = read_u16(iter.by_ref().cloned()).unwrap();
        let aligned_attr_len = align_attribute_len(attr_len as i32) as usize;
        let content = iter
            .by_ref()
            .take(aligned_attr_len - 4)
            .cloned()
            .collect::<Vec<_>>();

        let attr = match attr_type {
            IFLA_INFO_UNSPEC => LinkInfo::Unspecified(content),
            IFLA_INFO_KIND => LinkInfo::Kind(content),
            IFLA_INFO_DATA => LinkInfo::Data(content),
            IFLA_INFO_XSTATS => LinkInfo::ExtendedStats(content),
            IFLA_INFO_SLAVE_KIND => LinkInfo::SlaveKind(content),
            IFLA_INFO_SLAVE_DATA => LinkInfo::SlaveData(content),
            x @ _ => panic!("unknown LinkInfo type: {}", x),
        };

        Some((attr, aligned_attr_len))
    }

    pub fn serialize_in(&self, buffer: &mut Vec<u8>) {
        let original_len = buffer.len();

        // Push a length of zero into the buffer first
        buffer.extend(0u16.to_be_bytes().into_iter());

        // And then a type of zero second
        buffer.extend(0u16.to_be_bytes().into_iter());

        let attr_type;

        match self {
            Self::Unspecified(content) => {
                attr_type = IFLA_INFO_UNSPEC;
                buffer.extend(content.iter())
            }
            Self::Kind(content) => {
                attr_type = IFLA_INFO_KIND;
                buffer.extend(content.iter())
            }
            Self::Data(content) => {
                attr_type = IFLA_INFO_DATA;
                buffer.extend(content.iter())
            }
            Self::ExtendedStats(content) => {
                attr_type = IFLA_INFO_XSTATS;
                buffer.extend(content.iter())
            }
            Self::SlaveKind(content) => {
                attr_type = IFLA_INFO_SLAVE_KIND;
                buffer.extend(content.iter())
            }
            Self::SlaveData(content) => {
                attr_type = IFLA_INFO_SLAVE_DATA;
                buffer.extend(content.iter())
            }
        };

        for (i, byte) in (buffer.len() as u16).to_ne_bytes().into_iter().enumerate() {
            buffer[i] = byte;
        }

        for (i, byte) in attr_type.to_ne_bytes().into_iter().enumerate() {
            buffer[i + 2] = byte;
        }

        let length = (buffer.len() - original_len) as i32;
        for _ in 0..(align_attribute_len(length) - length) {
            buffer.push(0u8);
        }
    }
}
