// calculate all kinds of hashes

use std::path::Path;

pub struct Hashes;

impl Hashes {
    pub fn sha256<P: AsRef<Path> + Sync>(path: &P) -> anyhow::Result<String> {
        // load data from file
        let data = sha256::try_digest(path)?;
        Ok(sha256::digest(data))
    }
}
