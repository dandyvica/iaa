use bincode::Decode;
use serde::Serialize;

use crate::{
    discoverer::{Endianness, FileSignature},
    impl_discoverer,
};

//-------------------------------------------------------------------------------------------
// WAV
//-------------------------------------------------------------------------------------------
const SIGNATURE: FileSignature = FileSignature {
    header: b"BM",
    footer: None,
    mime: "bmp",
    endianness: Endianness::LittleEndian,
    metafunc: None,
};
impl_discoverer!(BMP, SIGNATURE);

#[derive(Debug, Serialize, Decode)]
pub struct BitmapFileHeaderAndCore {
    #[serde(skip)]
    pub bf_type: [u8; 2], // File type: "BM"

    pub bf_size: u32, // File size in bytes

    #[serde(skip)]
    pub bf_reserved1: u16, // Reserved, must be 0

    #[serde(skip)]
    pub bf_reserved2: u16, // Reserved, must be 0

    #[serde(skip)]
    pub bf_off_bits: u32, // Offset to pixel data

    pub bi_size: u32,             // Header size (40 bytes)
    pub bi_width: i32,            // Image width in pixels
    pub bi_height: i32,           // Image height (positive = bottom-up)
    pub bi_planes: u16,           // Must be 1
    pub bi_bit_count: u16,        // Bits per pixel
    pub bi_compression: u32,      // Compression type
    pub bi_size_image: u32,       // Image size (can be 0 for BI_RGB)
    pub bi_x_pels_per_meter: i32, // Horizontal resolution
    pub bi_y_pels_per_meter: i32, // Vertical resolution
    pub bi_clr_used: u32,         // Colors used
    pub bi_clr_important: u32,    // Important colors
}
