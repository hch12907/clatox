pub(crate) fn read_u16<I: Iterator<Item = u8>>(mut iter: I) -> Option<u16> {
    let bytes = [iter.next()?, iter.next()?];

    Some(u16::from_ne_bytes(bytes))
}

pub(crate) fn read_u32<I: Iterator<Item = u8>>(mut iter: I) -> Option<u32> {
    let bytes = [iter.next()?, iter.next()?, iter.next()?, iter.next()?];

    Some(u32::from_ne_bytes(bytes))
}
