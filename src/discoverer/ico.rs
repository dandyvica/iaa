use bincode::Decode;
use serde::Serialize;

use crate::{
    discoverer::{Endianness, FileSignature},
    impl_discoverer,
};

//-------------------------------------------------------------------------------------------
// ICO
//-------------------------------------------------------------------------------------------
const SIGNATURE: FileSignature = FileSignature {
    header: b"\x00\x00\x01\x00",
    footer: None,
    mime: "ico",
    endianness: Endianness::LittleEndian,
    metafunc: None,
};

impl_discoverer!(ICO, SIGNATURE);

#[repr(C)]
#[derive(Debug, Default, Serialize, Decode)]
pub struct IconDir {
    #[serde(skip)]
    pub id_reserved: u16, // Must be 0
    pub id_type: u16,  // 1 = ICO, 2 = CUR
    pub id_count: u16, // Number of images
}
