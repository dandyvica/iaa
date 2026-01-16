use bincode::Decode;
use nt_time::{time::UtcDateTime, FileTime};
use serde::{Serialize, Serializer};

use crate::{
    discoverer::{Endianness, FileSignature},
    impl_discoverer,
};

//-------------------------------------------------------------------------------------------
// PNG
//-------------------------------------------------------------------------------------------
const SIGN_REGF: FileSignature = FileSignature {
    header: b"regf",
    footer: None,
    mime: "regf",
    endianness: Endianness::LittleEndian,
    metafunc: None,
};

impl_discoverer!(REGF, SIGN_REGF);

#[derive(Debug, Serialize, Decode)]
pub struct RegistryBaseBlock {
    // ASCII string
    #[serde(skip)]
    signature: [u8; 4],

    // This number is incremented by 1 in the beginning of a write operation on the primary file
    primary_sequence_number: u32,

    // This number is incremented by 1 at the end of a write operation on the primary file, a *primary sequence number* and a *secondary sequence number* should be equal after a successful write operation
    secondary_sequence_number: u32,

    // FILETIME (UTC)
    #[serde(serialize_with = "timestamp_to_string")]
    last_written_timestamp: u64,

    // Major version of a hive writer
    major_version: u32,

    // Minor version of a hive writer
    minor_version: u32,

    // 0 means *primary file*
    file_type: u32,

    // 1 means *direct memory load*
    file_format: u32,

    // Offset of a root cell in bytes, relative from the start of the hive bins data
    root_cell_offset: u32,

    // Size of the hive bins data in bytes
    pub hive_bins_data_size: u32,

    // Logical sector size of the underlying disk in bytes divided by 512
    clustering_factor: u32,

    // UTF-16LE string (contains a partial file path to the primary file, or a file name of the primary file), used for debugging purposes
    #[serde(serialize_with = "utf16_to_string")]
    file_name: [u16; 32],
    // //
    // reserved1: [u8; 396],

    // // XOR-32 checksum of the previous 508 bytes
    // checksum: u32,

    // //
    // reserved2: [u8; 3576],

    // // This field has no meaning on a disk
    // boot_type: u32,

    // // This field has no meaning on a disk
    // boot_recover: u32,
}

// dedicated serializers
fn utf16_to_string<S>(values: &[u16; 32], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = String::from_utf16_lossy(values).replace('\0', "");
    serializer.serialize_str(&s)
}

fn timestamp_to_string<S>(windows_timestamp: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ft = FileTime::new(*windows_timestamp);
    match UtcDateTime::try_from(ft) {
        Ok(dt) => serializer.serialize_str(&dt.to_string()),
        Err(_) => serializer.serialize_str(&ft.to_string()),
    }
}
