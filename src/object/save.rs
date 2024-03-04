use std::fs;

use crate::{object::tree::Tree, REPO_ROOT};
use anyhow::Result;
use chrono::{DateTime, Utc};
use relative_path::RelativePath;
use sha2::{Digest, Sha256};

use super::Object;

pub struct Save {
    timestamp: DateTime<Utc>,
    digest: String,
    content: String,
}

impl Object for Save {
    fn digest(&self) -> &str {
        &self.digest
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn t<'a>(&self) -> &'a str {
        "save"
    }
}

pub fn execute(path: &RelativePath) -> Result<Save> {
    let tree = Tree::create(path)?;
    let timestamp = chrono::Utc::now();
    let mut content: String = String::new();

    content.push_str(&timestamp.to_rfc3339());
    content.push('\n');
    content.push_str(&format!("tree {} {}\n", path, tree.digest()));
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

    // set HEAD
    let head_path = std::env::current_dir()?.join(REPO_ROOT).join("HEAD");
    fs::write(head_path, &save.digest)?;

    Ok(save)
}
