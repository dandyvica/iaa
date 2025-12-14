use hex_literal::hex;
use serde::Serialize;
use serde_json::{Result, Value};

// list of usual file signatures
type SIGNATURE<'a> = (&'a [u8], Option<&'a [u8]>, &'static str);

const SIGN_SQLITE3: SIGNATURE = (
    b"\x53\x51\x4C\x69\x74\x65\x20\x66\x6F\x72\x6D\x61\x74\x20\x33\x00",
    None,
    "sqlite3",
);

#[allow(non_upper_case_globals)]
const SIGN_GIF87a: SIGNATURE = (
    b"\x47\x49\x46\x38\x37\x61",
    None,
    "GIF87a",
);

#[allow(non_upper_case_globals)]
const SIGN_GIF89a: SIGNATURE = (
    b"\x47\x49\x46\x38\x39\x61",
    None,
    "GIF89a",
);

// a macro for defining impl Discoverer
macro_rules! impl_discoverer {
    // Struct = struct name
    // Sign = tuple containing signatures
    ($Struct:ident, $Sign:ident) => {
        pub struct $Struct;
        impl<'a> Discoverer<'a> for $Struct {
            const HEADER: &'a [u8] = $Sign.0;
            const FOOTER: Option<&'a [u8]> = $Sign.1;
            const MIME: &'static str = $Sign.2;

            fn mime(bytes: &'a [u8]) -> Option<&'static str> {
                if bytes.len() >= Self::HEADER.len()
                    && bytes[0..Self::HEADER.len()] == *Self::HEADER
                {
                    Some(Self::MIME)
                } else {
                    None
                }
            }
        }
    };
}

// real implemntations
impl_discoverer!(SQLITE3, SIGN_SQLITE3);
impl_discoverer!(GIF87a, SIGN_GIF87a);
impl_discoverer!(GIF89a, SIGN_GIF89a);


// a trait for tyring to discover file types using magic numbers
pub trait Discoverer<'a> {
    const HEADER: &'a [u8];
    const FOOTER: Option<&'a [u8]>;
    const MIME: &'static str;

    // try to match header/footer on file bytes
    fn mime(bytes: &'a [u8]) -> Option<&'static str>;

    // try to get specific metadata
    // fn metadata(bytes: &'a [u8]) -> Option<String>;
}

// PNGs
pub struct PNG;

// the header chunk
#[derive(Debug, Default, Serialize)]
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

impl IHDR {
    // when reading the PNG file, we get access to raw data
    // this creates the header from bytes
    pub fn from_bytes(data: &[u8]) -> anyhow::Result<Self> {
        let mut ihdr = IHDR::default();

        let mut buf = [0u8; 4];
        buf.copy_from_slice(&data[0..4]);
        ihdr.width = u32::from_be_bytes(buf);

        buf.copy_from_slice(&data[4..8]);
        ihdr.height = u32::from_be_bytes(buf);

        ihdr.bit_depth = data[8];
        ihdr.color_type = data[9];
        ihdr.compression = data[10];
        ihdr.filter = data[11];
        ihdr.interlace = data[12];

        Ok(ihdr)
    }
}

impl<'a> Discoverer<'a> for PNG {
    const HEADER: &'a [u8] = &hex!("89 50 4E 47 0D 0A 1A 0A");
    const FOOTER: Option<&'a [u8]> = Some(&hex!("ae 42 60 82"));
    const MIME: &'static str = "png";

    fn mime(bytes: &'a [u8]) -> Option<&'static str> {
        let len = bytes.len();

        if len >= 12
            && bytes[..Self::HEADER.len()] == *Self::HEADER
            && bytes[len - 4..len] == *Self::FOOTER.unwrap()
        {
            Some(Self::MIME)
        } else {
            None
        }
    }

    // get IHDR chunk
    // fn metadata(bytes: &'a [u8]) -> Option<String> {
    //     let ihdr = IHDR::from_bytes(&bytes[16..29]).unwrap_or_default();
    //     Some(serde_json::to_string(&ihdr).unwrap())
    // }
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
