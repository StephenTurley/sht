use crate::object::tree::Tree;
use anyhow::Result;
use chrono::{DateTime, Utc};
use relative_path::RelativePath;
use sha2::{Digest, Sha256};

use super::Object;

pub struct Save {
    timestamp: DateTime<Utc>,
    digest: String,
    content: Vec<u8>,
}

impl Object for Save {
    fn digest(&self) -> &str {
        &self.digest
    }

    fn content(&self) -> &Vec<u8> {
        &self.content
    }

    fn t<'a>(&self) -> &'a str {
        "save"
    }
}

pub fn save_all(path: &RelativePath) -> Result<Save> {
    let tree = Tree::create(path)?;
    let timestamp = chrono::Utc::now();
    let mut content: Vec<u8> = Vec::new();

    // todo OMG JUST USE A STRING
    content.append(&mut timestamp.to_rfc3339().as_bytes().to_owned());
    content.append(&mut "\n".as_bytes().to_owned());
    content.append(
        &mut format!("tree {} {}\n", path, tree.digest())
            .as_bytes()
            .to_owned(),
    );
    let mut hasher = Sha256::new();

    hasher.update(&content);

    let digest = format!("{:x}", hasher.finalize());
    let save = Save {
        timestamp,
        content,
        digest,
    };
    save.write()?;

    tree.write_all()?;

    Ok(save)
}
