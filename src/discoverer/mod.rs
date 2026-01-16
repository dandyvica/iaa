use bincode::{config::BigEndian, Decode};
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

// all modules corresponding to file types here
// #tag
pub mod zip;
pub mod bmp;
pub mod gif;
pub mod ico;
pub mod png;
pub mod regf;
pub mod sqlite3;
pub mod wav;

// list of all file signatures
pub struct FileSignature {
    header: &'static [u8],
    footer: Option<&'static [u8]>,
    mime: &'static str,
    endianness: Endianness,
    metafunc: Option<fn(&[u8]) -> &[u8]>,
}

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
#[macro_export]
macro_rules! impl_discoverer {
    // Struct = struct name
    // Sign = tuple containing signatures
    ($Struct:ident, $Sign:ident) => {
        pub struct $Struct;
        impl<'a> crate::discoverer::Discoverer<'a> for $Struct {
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

#[cfg(test)]
mod tests {
    use crate::discoverer::png::PNG;
    use crate::discoverer::sqlite3::SQLITE3;

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
