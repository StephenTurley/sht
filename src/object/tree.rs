use std::{fs, path::Path};

use anyhow::Result;
use relative_path::RelativePath;
use sha2::{Digest, Sha256};

use super::{blob::Blob, write_object, Object, ObjectPath};
use crate::REPO_ROOT;

#[derive(Debug)]
pub struct Tree {
    digest: String,
    pub blobs: Vec<Entry<Blob>>,
    pub trees: Vec<Entry<Tree>>,
    content: String,
}

#[derive(Debug)]
pub struct Entry<T: Object> {
    pub path: String,
    pub object: T,
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

    fn write(&self) -> Result<()> {
        for entry in self.blobs.iter() {
            entry.object.write()?;
        }
        for entry in self.trees.iter() {
            entry.object.write()?;
        }

        write_object(self)?;
        Ok(())
    }
}

impl Tree {
    pub fn create(path: &RelativePath) -> Result<Tree> {
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

    pub fn load(hash: &str) -> Result<Tree> {
        let file = ObjectPath::from(hash).file_name;
        let content = fs::read_to_string(file)?;
        let lines = content.lines();
        let mut blobs: Vec<Entry<Blob>> = Vec::new();
        let mut trees: Vec<Entry<Tree>> = Vec::new();
        for line in lines {
            let mut parts = line.split_whitespace();
            let t = parts.next().unwrap();
            let path = parts.next().unwrap();
            let hash = parts.next().unwrap();
            match t {
                "blob" => {
                    let blob = Blob::load(hash)?;
                    blobs.push(Entry {
                        path: path.to_string(),
                        object: blob,
                    });
                }
                "tree" => {
                    let tree = Tree::load(hash)?;
                    trees.push(Entry {
                        path: path.to_string(),
                        object: tree,
                    });
                }
                _ => {}
            }
        }
        Ok(Tree {
            blobs,
            trees,
            content,
            digest: hash.to_string(),
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
