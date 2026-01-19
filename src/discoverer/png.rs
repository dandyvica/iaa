use bincode::Decode;
use serde::Serialize;

use crate::{
    discoverer::{Endianness, FileSignature},
    impl_discoverer,
};

//-------------------------------------------------------------------------------------------
// PNG
//-------------------------------------------------------------------------------------------
const SIGNATURE: FileSignature = FileSignature {
    header: b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A",
    footer: Some(b"\xae\x42\x60\x82"),
    mime: "png",
    endianness: Endianness::BigEndian,
    //metafunc: Some(|x| &x[16..29]),
};

impl_discoverer!(PNG, SIGNATURE);

#[repr(C)]
#[derive(Debug, Default, Serialize, Decode)]
/// Represents the IHDR chunk of a PNG image.
/// This is the first chunk in every valid PNG file and defines
/// the basic characteristics of the image.
pub struct IHDR {
    #[serde(skip)]
    signature: [u8; 8],

    #[serde(skip)]
    ihdr_length: u32,

    #[serde(skip)]
    ihdr_type: u32,

    /// Image width in pixels (must be greater than 0).
    pub width: u32,

    /// Image height in pixels (must be greater than 0).
    pub height: u32,

    /// Bit depth — number of bits per sample or per palette index.
    /// Common values: 1, 2, 4, 8, or 16, depending on color type.
    pub bit_depth: u8,

    /// Color type — indicates how pixel data is interpreted.
    /// 0: Grayscale  
    /// 2: Truecolor (RGB)  
    /// 3: Indexed-color (palette)  
    /// 4: Grayscale with alpha  
    /// 6: Truecolor with alpha
    pub color_type: u8,

    /// Compression method — must be 0 in valid PNGs (deflate/inflate).
    pub compression: u8,

    /// Filter method — must be 0 (the standard adaptive filtering method).
    pub filter: u8,

    /// Interlace method:
    /// 0: No interlace  
    /// 1: Adam7 interlace
    pub interlace: u8,
}
