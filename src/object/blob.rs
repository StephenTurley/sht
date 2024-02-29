use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

use super::Object;
use anyhow::Result;

#[derive(Debug)]
pub struct Blob {
    digest: String,
    content: String,
}

impl Object for Blob {
    fn digest(&self) -> &str {
        &self.digest
    }

    fn t<'a>(&self) -> &'a str {
        "blob"
    }

    fn content(&self) -> &str {
        &self.content
    }
}

impl Blob {
    pub fn create(path: &PathBuf) -> Result<Blob> {
        let mut hasher = Sha256::new();
        let content: String = fs::read_to_string(path)?;
        hasher.update(&content);

        let digest = format!("{:x}", hasher.finalize());
        Ok(Blob { digest, content })
    }
}
