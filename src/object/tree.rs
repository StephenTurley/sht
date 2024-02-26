use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use relative_path::RelativePath;
use sha2::{Digest, Sha256};

use super::{blob::Blob, Object};
use crate::REPO_ROOT;

#[derive(Debug)]
pub struct Tree {
    digest: String,
    blobs: HashMap<String, Blob>,
    trees: HashMap<String, Tree>,
    content: Vec<u8>,
}

impl Object for Tree {
    fn digest(&self) -> &str {
        &self.digest
    }

    fn t<'a>(&self) -> &'a str {
        "tree"
    }

    fn content(&self) -> &Vec<u8> {
        &self.content
    }
}

impl Tree {
    pub fn write_all(&self) -> Result<()> {
        for blob in self.blobs.values() {
            blob.write()?;
        }
        for tree in self.trees.values() {
            tree.write_all()?;
        }
        self.write()?;
        Ok(())
    }
    pub fn create(path: &RelativePath) -> Result<Tree> {
        let mut blobs: HashMap<String, Blob> = HashMap::new();
        let mut trees: HashMap<String, Tree> = HashMap::new();
        let mut content: Vec<u8> = Vec::new();

        for entry in fs::read_dir(path.to_path(""))? {
            let entry = entry?;
            let path = entry.path();
            let rel_path = RelativePath::from_path(&path)?;
            if path.is_dir() {
                if path != Path::new(REPO_ROOT) {
                    trees.insert(rel_path.to_string(), Self::create(rel_path)?);
                }
            } else {
                let blob = Blob::create(&path)?;
                blobs.insert(rel_path.to_string(), blob);
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

fn append_content<T: Object>(s: &mut Vec<u8>, entries: &HashMap<String, T>) {
    for (path, object) in entries.iter() {
        let entry_s = format!("{} {} {}\n", object.t(), path, object.digest());
        s.append(&mut entry_s.as_bytes().to_owned());
    }
}
