use bincode::{config::BigEndian, Decode};
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

// list of all file signatures
pub struct FileSignature {
    header: &'static [u8],
    footer: Option<&'static [u8]>,
    mime: &'static str,
    endianness: Endianness,
    metafunc: Option<fn(&[u8]) -> &[u8]>,
}

const SIGN_SQLITE3: FileSignature = FileSignature {
    header: b"\x53\x51\x4C\x69\x74\x65\x20\x66\x6F\x72\x6D\x61\x74\x20\x33\x00",
    footer: None,
    mime: "sqlite3",
    endianness: Endianness::BigEndian,
    metafunc: None,
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

// to get some metadat, we have to know whether integers are stored
// using big or little endian. E.g: PNG uses big endian
#[derive(Debug, PartialEq)]
enum Endianness {
    BigEndian,
    LittleEndian,
}

// a trait for tyring to discover file types using magic numbers
pub trait Discoverer<'a> {
    const FILE_SIGNATURE: FileSignature;

    // try to match header/footer on file bytes
    fn mime(bytes: &'a [u8]) -> Option<&'static str>;

    // try to get specific metadata
    fn metadata<T: Serialize + Decode<()>>(bytes: &'a [u8]) -> Option<serde_json::Value> {
        // bincode config differs whether we decode big_endian or little_endian ints
        let bytes = if let Some(metafunc) = Self::FILE_SIGNATURE.metafunc {
            metafunc(bytes)
        } else {
            bytes
        };

        if Self::FILE_SIGNATURE.endianness == Endianness::BigEndian {
            let config = bincode::config::standard()
                .with_big_endian()
                .with_fixed_int_encoding();
            let decoded: (T, usize) = bincode::decode_from_slice(bytes, config).ok()?;
            serde_json::to_value(&decoded.0).ok()
        } else {
            let config = bincode::config::standard()
                .with_little_endian()
                .with_fixed_int_encoding();
            let decoded: (T, usize) = bincode::decode_from_slice(bytes, config).ok()?;
            serde_json::to_value(&decoded.0).ok()
        }
    }
}

// a macro for defining impl Discoverer
macro_rules! impl_discoverer {
    // Struct = struct name
    // Sign = tuple containing signatures
    ($Struct:ident, $Sign:ident) => {
        pub struct $Struct;
        impl<'a> Discoverer<'a> for $Struct {
            const FILE_SIGNATURE: FileSignature = $Sign;

            fn mime(bytes: &'a [u8]) -> Option<&'static str> {
                let len = bytes.len();
                let fs = Self::FILE_SIGNATURE;

                // 2 cases: we've got only the header, or also footer
                // both
                if let Some(footer) = fs.footer {
                    if len >= (fs.header.len() + footer.len())
                        && bytes[..fs.header.len()] == *fs.header
                        && bytes[len - footer.len()..len] == *footer
                    {
                        Some(fs.mime)
                    } else {
                        None
                    }
                }
                // only header
                else {
                    if len >= fs.header.len() && bytes[0..fs.header.len()] == *fs.header {
                        Some(fs.mime)
                    } else {
                        None
                    }
                }
            }
        }
    };
}

// list of usual file signatures
// - bytes header
// - optional bytes for footer
// - mime type
// - endianness
type SIGNATURE<'a> = (&'a [u8], Option<&'a [u8]>, &'static str, Endianness);

// real implemntations
impl_discoverer!(SQLITE3, SIGN_SQLITE3);
impl_discoverer!(GIF87a, SIGN_GIF87a);
impl_discoverer!(GIF89a, SIGN_GIF89a);


//-------------------------------------------------------------------------------------------
// PNG
//-------------------------------------------------------------------------------------------
const SIGN_PNG: FileSignature = FileSignature {
    header: b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A",
    footer: Some(b"\xae\x42\x60\x82"),
    mime: "png",
    endianness: Endianness::BigEndian,
    metafunc: Some(|x| &x[16..29]),
};

impl_discoverer!(PNG, SIGN_PNG);

#[derive(Debug, Default, Serialize, Decode)]
/// Represents the IHDR chunk of a PNG image.
/// This is the first chunk in every valid PNG file and defines
/// the basic characteristics of the image.
pub struct IHDR {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn png() -> anyhow::Result<()> {
        let buffer = std::fs::read("tests/test.png")?;
        assert_eq!(PNG::mime(&buffer), Some("png"));

        let buffer = std::fs::read("tests/test.jpg")?;
        assert!(PNG::mime(&buffer).is_none());

        Ok(())
    }

    #[test]
    fn sqlite3() -> anyhow::Result<()> {
        let buffer = std::fs::read("tests/test.db")?;
        assert_eq!(SQLITE3::mime(&buffer), Some("sqlite3"));

        let buffer = std::fs::read("tests/test.jpg")?;
        assert!(SQLITE3::mime(&buffer).is_none());

        Ok(())
    }
}
