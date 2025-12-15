use std::{fs::File, io, ops::Deref, path::Path};

use memmap::Mmap;

use crate::discoverer::{Discoverer, IHDR};

macro_rules! try_discover {
    // case of non metadata are possible to extract (or too difficult)
    ($Struct:ident, $Self:ident) => {
        use crate::discoverer::$Struct;

        if let Some(value) = $Struct::mime($Self) {
            return (Some(value), None);
        }
    };

    // case of metadata are possible to extract
    ($Struct:ident, $Self:ident, $MetaStruct:ident) => {
        use crate::discoverer::$Struct;

        if let Some(value) = $Struct::mime($Self) {
            let metadata = $Struct::metadata::<$MetaStruct>($Self);
            return (Some(value), metadata);
        }
    };
}

// as we have to calculate hashes, magic number etc, w use memmap
// to load data in to memory
pub struct MappedFile(Mmap);

// load file from path into memory
impl TryFrom<&Path> for MappedFile {
    type Error = io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        // open file
        let file = File::open(path)?;

        // mmemap is unsafe by nature
        let mmap = unsafe { Mmap::map(&file)? };

        Ok(Self(mmap))
    }
}

// now we have mapped the file, we have data in memory
impl MappedFile {
    // Blake3 hash
    pub fn blake3(&self) -> String {
        blake3::hash(self).to_string()
    }

    // sah256 hash
    pub fn sha256(&self) -> String {
        // load data from file
        sha256::digest(self.as_ref())
    }

    // calculate the Shannon entropy
    pub fn entropy(&self) -> f32 {
        entropy::shannon_entropy(self.as_ref())
    }

    // try to discover mime type from magic numbers
    pub fn discover(&self) -> (Option<&'static str>, Option<serde_json::Value>) {
        try_discover!(PNG, self, IHDR);
        try_discover!(SQLITE3, self);
        try_discover!(GIF87a, self);
        try_discover!(GIF89a, self);

        (None, None)
    }
}

impl Deref for MappedFile {
    type Target = Mmap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
