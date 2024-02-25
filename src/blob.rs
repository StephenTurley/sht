use sha2::{Digest, Sha256};
use std::fs;

use anyhow::Result;

pub fn create(file: &str) -> Result<()> {
    let mut hasher = Sha256::new();
    let contents = fs::read(file)?;
    hasher.update(&contents);

    // todo build this using Path so it works on other all OSs.
    // also it should not create the blob if it exists
    // should add the "blob" header so we can figure out what it is later
    let digest = format!("{:x}", hasher.finalize());
    let dir = format!("./.sht/objects/{}", &digest[0..3]);

    fs::create_dir_all(&dir)?;
    fs::write(format!("{}/{}", &dir, &digest[3..0]), contents)?;
    Ok(())
}
