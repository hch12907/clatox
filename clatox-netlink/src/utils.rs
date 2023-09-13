//! The utilities here are provided to assist implementation of various Netlink
//! protocols.

use libc::NLA_ALIGNTO;

/// Reads 2 bytes from the iterator to form a u16 integer.
pub fn read_u16<I: Iterator<Item = u8>>(mut iter: I) -> Option<u16> {
    let bytes = [iter.next()?, iter.next()?];

    Some(u16::from_ne_bytes(bytes))
}

/// Reads 4 bytes from the iterator to form a u32 integer.
pub fn read_u32<I: Iterator<Item = u8>>(mut iter: I) -> Option<u32> {
    let bytes = [iter.next()?, iter.next()?, iter.next()?, iter.next()?];

    Some(u32::from_ne_bytes(bytes))
}

/// Calculates the actual length of an attribute. For instance, an attribute
/// that is 7 bytes in length might be 8 bytes long (theoretically, it can be
/// even longer if the system demands it to be).
/// 
/// On an x86_64 Linux system, this most likely aligns to 4 bytes.
pub fn align_attribute_len(len: i32) -> i32 {
    (len + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)
}

/// Serializes an attribute into a buffer. The serialized attribute will be
/// aligned to byte multiples according to `align_attribute_len()`.
/// 
/// The function closure accepts a buffer in which it can write the attribute
/// payload into, and returns the type of the written attribute. 
pub fn serialize_attribute_into<F>(buffer: &mut Vec<u8>, f: F)
where
    F: Fn(&mut Vec<u8>) -> u16
{
    let original_len = buffer.len();

    // Push a length of zero into the buffer first
    buffer.extend(0u16.to_be_bytes().into_iter());

    // And then a type of zero second
    buffer.extend(0u16.to_be_bytes().into_iter());

    let attr_type = f(buffer);

    let length = buffer.len() - original_len;

    for (i, byte) in (length as u16).to_ne_bytes().into_iter().enumerate() {
        buffer[i + original_len] = byte;
    }

    for (i, byte) in attr_type.to_ne_bytes().into_iter().enumerate() {
        buffer[i + original_len + 2] = byte;
    }

    for _ in 0..(align_attribute_len(length as i32) - length as i32) {
        buffer.push(0u8);
    }
}
