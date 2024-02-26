use sha2::{Digest, Sha256};
use std::io::Write;
use std::{fs, path::PathBuf};

use anyhow::Result;

use crate::REPO_ROOT;

pub struct Blob {
    digest: String,
    content: Vec<u8>,
}

impl Blob {
    pub fn write(&self) -> Result<()> {
        let blob_path = std::env::current_dir()?
            .join(REPO_ROOT)
            .join("objects/")
            .join(&self.digest[0..3]);

        fs::create_dir_all(&blob_path)?;
        let file_name = blob_path.join(&self.digest[3..]);
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_name)?;

        file.write_all(&self.content)?;
        Ok(())
    }
    pub fn create(path: &PathBuf) -> Result<Blob> {
        let mut hasher = Sha256::new();
        let mut content: Vec<u8> = Vec::new();

        content.append(&mut "blob\n".as_bytes().to_owned());
        content.append(&mut fs::read(path)?);
        hasher.update(&content);

        let digest = format!("{:x}", hasher.finalize());
        Ok(Blob { digest, content })
    }
}
