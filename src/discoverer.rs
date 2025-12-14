use hex_literal::hex;
use serde::Serialize;
use serde_json::{Result, Value};

// a trait for tyring to discover file types using magic numbers
pub trait Discoverer<'a> {
    const HEADER: &'a [u8];
    const FOOTER: &'a [u8];
    const MIME: &'static str;

    // try to match header/footer on file bytes
    fn mime(bytes: &'a [u8]) -> Option<&'static str>;

    // try to get specific metadata
    fn metadata(bytes: &'a [u8]) -> Option<String>;
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
    const FOOTER: &'a [u8] = &hex!("ae 42 60 82");
    const MIME: &'static str = "png";

    fn mime(bytes: &'a [u8]) -> Option<&'static str> {
        let len = bytes.len();

        if len >= 12
            && bytes[..Self::HEADER.len()] == *Self::HEADER
            && bytes[len - 4..len] == *Self::FOOTER
        {
            Some(Self::MIME)
        } else {
            None
        }
    }

    // get IHDR chunk
    fn metadata(bytes: &'a [u8]) -> Option<String> {
        let ihdr = IHDR::from_bytes(&bytes[16..29]).unwrap_or_default();
        Some(serde_json::to_string(&ihdr).unwrap())
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn png() -> anyhow::Result<()> {
        let buffer = std::fs::read("tests/nemo.png")?;
        assert_eq!(PNG::mime(&buffer), Some("png"));

        let buffer = std::fs::read("tests/example-image.jpg")?;
        assert!(PNG::mime(&buffer).is_none());

        Ok(())
    }
}
