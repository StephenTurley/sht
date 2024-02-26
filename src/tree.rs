use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use relative_path::RelativePath;

use crate::{blob::Blob, REPO_ROOT};

pub struct Tree {
    digest: String,
    blobs: HashMap<String, Blob>,
    trees: HashMap<String, Tree>,
    content: String,
}

pub fn create(path: &RelativePath) -> Result<Tree> {
    let mut blobs: HashMap<String, Blob> = HashMap::new();
    let mut trees: HashMap<String, Tree> = HashMap::new();

    for entry in fs::read_dir(path.to_path(""))? {
        let entry = entry?;
        let path = entry.path();
        let rel_path = RelativePath::from_path(&path)?;
        if path.is_dir() {
            if path != Path::new(REPO_ROOT) {
                trees.insert(rel_path.to_string(), create(rel_path)?);
            }
        } else {
            let blob = Blob::create(&path)?;
            blobs.insert(rel_path.to_string(), blob);
        }
    }
    Ok(Tree {
        blobs,
        trees,
        content: "".to_string(),
        digest: "".to_string(),
    })
}

// todo create content, digest , and add a write function that saves the entire tree to disk
