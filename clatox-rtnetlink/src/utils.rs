use libc::NLA_ALIGNTO;

pub(crate) fn read_u16<I: Iterator<Item = u8>>(mut iter: I) -> Option<u16> {
    let bytes = [iter.next()?, iter.next()?];

    Some(u16::from_ne_bytes(bytes))
}

pub(crate) fn read_u32<I: Iterator<Item = u8>>(mut iter: I) -> Option<u32> {
    let bytes = [iter.next()?, iter.next()?, iter.next()?, iter.next()?];

    Some(u32::from_ne_bytes(bytes))
}

pub(crate) fn align_attribute_len(len: i32) -> i32 {
    // TODO: we shouldn't use NLA_ALIGNTO here.
    (len + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)
}
