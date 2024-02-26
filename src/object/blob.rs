use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

use super::Object;
use anyhow::Result;

#[derive(Debug)]
pub struct Blob {
    digest: String,
    content: Vec<u8>,
}

impl Object for Blob {
    fn digest(&self) -> &str {
        &self.digest
    }

    fn t<'a>(&self) -> &'a str {
        "blob"
    }

    fn content(&self) -> &Vec<u8> {
        &self.content
    }
}

impl Blob {
    pub fn create(path: &PathBuf) -> Result<Blob> {
        let mut hasher = Sha256::new();
        let mut content: Vec<u8> = Vec::new();

        content.append(&mut fs::read(path)?);
        hasher.update(&content);

        let digest = format!("{:x}", hasher.finalize());
        Ok(Blob { digest, content })
    }
}
