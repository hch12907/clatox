use crate::utils::{read_u16, align_attribute_len};

/// A trait implemented by all Netlink message attributes.
pub trait Attribute: Sized {
    fn from_raw(raw: RawAttribute) -> Option<Self>;

    // Do we want this? Converting serialize_into() to to_raw() is not super
    // straightforward unlike deserialize() and from_raw().
    // fn to_raw(&self) -> RawAttribute;
}

/// A raw Netlink attribute consisting of length, type, and associated payload.
#[repr(C)]
pub struct RawAttribute {
    attr_len: u16,
    attr_type: u16,
    payload: Vec<u8>,
}

impl RawAttribute {
    /// Create an attribute according to the attribute type and the payload.
    /// 
    /// Panics
    /// ======
    /// If `payload.len() + 4` exceeds `u16::MAX`, this function panics.
    /// 
    pub fn new(attr_type: u16, payload: Vec<u8>) -> Self {
        assert!(payload.len() + 4 < u16::MAX as usize);

        Self {
            attr_len: (4 + payload.len()) as u16,
            attr_type,
            payload
        }
    }

    /// The length of an attribute. It need not be aligned.
    /// 
    /// Note that this includes the attribute header itself!
    pub fn length(&self) -> u16 {
        self.attr_len
    }

    /// The type of an attribute.
    pub fn attr_type(&self) -> u16 {
        self.attr_type
    }

    /// The payload of an attribute.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn into_payload(self) -> Vec<u8> {
        self.payload
    }
}

/// This iterator generates Netlink message attributes from a stream of bytes.
/// 
/// Each attribute will contain `(length: u16, type: u16, payload: [u8; length - 4])`
/// bytes, followed by optional padding (see [`align_attribute_len`]).
pub struct RawAttributeIter<I: Iterator<Item = u8>> {
    bytes: I
}

impl<I: Iterator<Item = u8>> RawAttributeIter<I> {
    pub fn new(iter: I) -> Self {
        Self { bytes: iter }
    }
}

impl<I: Iterator<Item = u8>> Iterator for RawAttributeIter<I> {
    type Item = RawAttribute;

    fn next(&mut self) -> Option<Self::Item> {
        let attr_len = read_u16(self.bytes.by_ref())?;
        let attr_type = read_u16(self.bytes.by_ref()).unwrap();
        let aligned_attr_len = align_attribute_len(attr_len as i32) as usize;
        let mut payload = self.bytes
            .by_ref()
            .take(aligned_attr_len - 4)
            .collect::<Vec<_>>();

        payload.truncate(attr_len as usize - 4);

        Some(RawAttribute{
            attr_len,
            attr_type,
            payload,
        })
    }
}
