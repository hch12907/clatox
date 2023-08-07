use libc::*;

use crate::utils::{align_attribute_len, read_u16};

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
        let mut iter = bytes.iter();

        let attr_len = read_u16(iter.by_ref().cloned()).unwrap();
        let attr_type = read_u16(iter.by_ref().cloned()).unwrap();
        let aligned_attr_len = align_attribute_len(attr_len as i32) as usize;
        let content = iter
            .by_ref()
            .take(aligned_attr_len - 4)
            .cloned()
            .collect::<Vec<_>>();

        let attr = match attr_type as i32 {
            AF_INET => AddressFamilySpecific::Inet(content),
            AF_INET6 => AddressFamilySpecific::Inet6(content),
            typ => AddressFamilySpecific::Other(typ as u8, content),
        };

        Some((attr, aligned_attr_len))
    }

    pub fn serialize_in(&self, buffer: &mut Vec<u8>) {
        let original_len = buffer.len();

        // Push a length of zero into the buffer first
        buffer.extend(0u16.to_be_bytes().into_iter());

        // And then a type of zero second
        buffer.extend(0u16.to_be_bytes().into_iter());

        let attr_type: u16;

        match self {
            Self::Inet(content) => {
                attr_type = AF_INET as u16;
                buffer.extend(content.iter())
            }
            Self::Inet6(content) => {
                attr_type = AF_INET6 as u16;
                buffer.extend(content.iter())
            }
            Self::Other(typ, content) => {
                attr_type = *typ as u16;
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
