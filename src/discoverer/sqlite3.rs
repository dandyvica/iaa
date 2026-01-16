use crate::{
    discoverer::{Endianness, FileSignature},
    impl_discoverer,
};

const SIGN_SQLITE3: FileSignature = FileSignature {
    header: b"\x53\x51\x4C\x69\x74\x65\x20\x66\x6F\x72\x6D\x61\x74\x20\x33\x00",
    footer: None,
    mime: "sqlite3",
    endianness: Endianness::BigEndian,
    metafunc: None,
};

impl_discoverer!(SQLITE3, SIGN_SQLITE3);
