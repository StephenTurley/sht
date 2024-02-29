use std::{fs, path::Path};

use anyhow::Result;
use relative_path::RelativePath;
use sha2::{Digest, Sha256};

use super::{blob::Blob, Object};
use crate::REPO_ROOT;

#[derive(Debug)]
pub struct Tree {
    digest: String,
    blobs: Vec<Entry<Blob>>,
    trees: Vec<Entry<Tree>>,
    content: String,
}

#[derive(Debug)]
pub struct Entry<T: Object> {
    path: String,
    object: T,
}

impl Object for Tree {
    fn digest(&self) -> &str {
        &self.digest
    }

    fn t<'a>(&self) -> &'a str {
        "tree"
    }

    fn content(&self) -> &str {
        &self.content
    }
}

impl Tree {
    pub fn write_all(&self) -> Result<()> {
        for entry in self.blobs.iter() {
            entry.object.write()?;
        }
        for entry in self.trees.iter() {
            entry.object.write_all()?;
        }
        self.write()?;
        Ok(())
    }
    pub fn create(path: &RelativePath) -> Result<Tree> {
        // todo this needs to always be in the same order
        let mut blobs: Vec<Entry<Blob>> = Vec::new();
        let mut trees: Vec<Entry<Tree>> = Vec::new();
        let mut content: String = String::new();

        for entry in fs::read_dir(path.to_path(""))? {
            let entry = entry?;
            let path = entry.path();
            let rel_path = RelativePath::from_path(&path)?;
            if path.is_dir() {
                if path != Path::new(REPO_ROOT) {
                    trees.push(Entry {
                        path: rel_path.to_string(),
                        object: Self::create(rel_path)?,
                    });
                }
            } else {
                let blob = Blob::create(&path)?;
                blobs.push(Entry {
                    path: rel_path.to_string(),
                    object: blob,
                });
            }
        }
        append_content(&mut content, &blobs);
        append_content(&mut content, &trees);
        let mut hasher = Sha256::new();

        hasher.update(&content);

        let digest = format!("{:x}", hasher.finalize());

        Ok(Tree {
            blobs,
            trees,
            content,
            digest,
        })
    }
}

// maybe reuse this in Save?
fn append_content<T: Object>(s: &mut String, entries: &[Entry<T>]) {
    for entry in entries.iter() {
        let entry_s = format!(
            "{} {} {}\n",
            entry.object.t(),
            entry.path,
            entry.object.digest()
        );
        s.push_str(&entry_s);
    }
}
