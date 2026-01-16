use bincode::Decode;
use serde::Serialize;

use crate::{
    discoverer::{Endianness, FileSignature},
    impl_discoverer,
};

//-------------------------------------------------------------------------------------------
// ZIP
//-------------------------------------------------------------------------------------------
const SIGNATURE: FileSignature = FileSignature {
    header: b"PK\x03\x04",
    footer: None,
    mime: "zip",
    endianness: Endianness::LittleEndian,
    metafunc: Some(|x| &x[16..29]),
};

impl_discoverer!(ZIP, SIGNATURE);

#[repr(C)]
#[derive(Debug, Default, Serialize, Decode)]
pub struct ZipFile {
    pub files: Vec<String>,
}


