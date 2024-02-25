use sha2::{Digest, Sha256};
use std::io::Write;
use std::{fs, path::PathBuf};

use anyhow::Result;

use crate::REPO_ROOT;

pub fn create(path: &PathBuf) -> Result<()> {
    let mut hasher = Sha256::new();
    let header = "blob\n".as_bytes().to_owned();
    let contents = fs::read(path)?;
    hasher.update(&header);
    hasher.update(&contents);

    // todo build this using Path so it works on other all OSs.
    // also it should not create the blob if it exists
    // should add the "blob" header so we can figure out what it is later
    let digest = format!("{:x}", hasher.finalize());
    // let dir = format!("{}/objects/{}", REPO_ROOT, &digest[0..3]);
    let blob_path = std::env::current_dir()?
        .join(REPO_ROOT)
        .join("objects/")
        .join(&digest[0..3]);

    fs::create_dir_all(&blob_path)?;
    let file_name = blob_path.join(&digest[3..]);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)?;

    file.write_all(&header)?;
    file.write_all(&contents)?;

    Ok(())
}
