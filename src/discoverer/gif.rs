use crate::{
    discoverer::{Endianness, FileSignature},
    impl_discoverer,
};

#[allow(non_upper_case_globals)]
const SIGN_GIF87a: FileSignature = FileSignature {
    header: b"\x47\x49\x46\x38\x37\x61",
    footer: None,
    mime: "GIF87a",
    endianness: Endianness::LittleEndian,
    metafunc: None,
};

#[allow(non_upper_case_globals)]
const SIGN_GIF89a: FileSignature = FileSignature {
    header: b"\x47\x49\x46\x38\x39\x61",
    footer: None,
    mime: "GIF89a",
    endianness: Endianness::LittleEndian,
    metafunc: None,
};

impl_discoverer!(GIF87a, SIGN_GIF87a);
impl_discoverer!(GIF89a, SIGN_GIF89a);
