use std::io::Cursor;

use bincode::Decode;
use serde::Serialize;
use zip::ZipArchive;

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
};

impl_discoverer!(ZIP, SIGNATURE);

#[repr(C)]
#[derive(Debug, Default, Serialize, Decode)]
pub struct ZipFile {
    pub files: Vec<String>,
}

// for some types, we'll not call the FileSignature trait's metadata()
// but a custom one, as Rust doesn't yet support trait's method specialization
#[derive(Debug, Serialize)]
struct ZipMeta {
    // zip entry name
    name: String,

    // zip entry size
    size: u64,
}

impl ZIP {
    // give the list of files in the archive
    pub fn files(bytes: &[u8]) -> Option<serde_json::Value> {
        let mut v = Vec::new();

        // zip APIs operates on readers
        let c = Cursor::new(bytes);
        let mut archive = ZipArchive::new(c).ok()?;

        for i in 0..archive.len() {
            let entry = archive.by_index(i).ok()?;

            v.push(ZipMeta {
                name: entry.name().to_string(),
                size: entry.size(),
            });
        }

        serde_json::to_value(v).ok()
    }
}
