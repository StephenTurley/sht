use std::fs;

use crate::object::tree::{self, Tree};
use anyhow::Result;
use chrono::{DateTime, Utc};
use relative_path::RelativePath;
use sha2::{Digest, Sha256};

use super::{write_object, Object, ObjectPath};

pub struct Save {
    // TODO remove this when log implemented
    #[allow(dead_code)]
    timestamp: DateTime<Utc>,
    pub tree: Tree,
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

    fn write(&self) -> Result<()> {
        write_object(self)?;
        self.tree.write()?;
        Ok(())
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
