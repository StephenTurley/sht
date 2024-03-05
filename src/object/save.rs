use std::fs;

use crate::{
    object::tree::{self, Tree},
    REPO_ROOT,
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use relative_path::RelativePath;
use sha2::{Digest, Sha256};

use super::{Object, ObjectPath};

pub struct Save {
    // TODO remove this when log implemented
    #[allow(dead_code)]
    timestamp: DateTime<Utc>,
    tree: Tree,
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

impl Save {
    pub fn create(path: &RelativePath) -> Result<Save> {
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
            tree,
            digest,
            content,
        };

        Ok(save)
    }

    pub fn write_all(&self) -> Result<()> {
        self.write()?;
        self.tree.write_all()?;
        Ok(())
    }

    pub fn load(hash: &str) -> Result<Save> {
        let file = ObjectPath::from(hash).file_name;
        let content = fs::read_to_string(file)?;
        let mut lines = content.lines();
        let timestamp: DateTime<Utc> = lines
            .next()
            .unwrap()
            .parse()
            .expect("Could not parse timestamp");

        let tree_line = lines.next().unwrap();
        let tree_parts: Vec<&str> = tree_line.split_whitespace().collect();
        let tree = tree::Tree::load(tree_parts[2])?;

        Ok(Save {
            timestamp,
            tree,
            digest: hash.to_string(),
            content,
        })
    }
}

pub fn execute(path: &RelativePath) -> Result<Save> {
    let new_save = Save::create(path)?;
    let head_path = std::env::current_dir()?.join(REPO_ROOT).join("HEAD");

    if head_path.exists() {
        let head = fs::read_to_string(head_path)?;
        let existing_save = Save::load(&head)?;
        if existing_save.tree.digest() != new_save.tree.digest() {
            save_new_head(&new_save)?;
            Ok(new_save)
        } else {
            Ok(existing_save)
        }
    } else {
        save_new_head(&new_save)?;
        Ok(new_save)
    }
}

fn save_new_head(new_save: &Save) -> Result<()> {
    let head_path = std::env::current_dir()?.join(REPO_ROOT).join("HEAD");
    new_save.write_all()?;
    fs::write(head_path, &new_save.digest)?;
    Ok(())
}
