use bincode::Decode;
use serde::Serialize;

use crate::{
    discoverer::{Endianness, FileSignature, LITTLEENDIAN_CONFIG},
    impl_discoverer,
};

//-------------------------------------------------------------------------------------------
// WAV
//-------------------------------------------------------------------------------------------
const SIGNATURE: FileSignature = FileSignature {
    header: b"RIFF",
    footer: None,
    mime: "wav",
    endianness: Endianness::LittleEndian,
    // metafunc: Some(struct_decoder),
};

impl_discoverer!(WAV, SIGNATURE);

#[repr(C)]
#[derive(Debug, Serialize, Decode)]
pub struct WavHeader {
    #[serde(skip)]
    pub riff_id: [u8; 4], // "RIFF"

    pub riff_size: u32, // file size - 8

    #[serde(skip)]
    pub wave_id: [u8; 4], // "WAVE"

    // fmt subchunk
    #[serde(skip)]
    pub fmt_id: [u8; 4], // "fmt "
    pub fmt_size: u32,        // PCM = 16
    pub audio_format: u16,    // PCM = 1
    pub num_channels: u16,    // 1 = mono, 2 = stereo, etc.
    pub sample_rate: u32,     // e.g. 44100
    pub byte_rate: u32,       // sample_rate * num_channels * bits_per_sample/8
    pub block_align: u16,     // num_channels * bits_per_sample/8
    pub bits_per_sample: u16, // e.g. 16
}
