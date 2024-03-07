use std::fs;

use anyhow::Result;
use relative_path::RelativePath;

use crate::object::{save::Save, tree::Tree, Object};

pub struct Status {
    pub modified: Vec<String>,
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

pub fn execute() -> Result<Status> {
    let head = fs::read_to_string(".sht/HEAD")?;
    let save = Save::load(head.trim())?;

    let saved_tree = save.tree;
    let current_tree = Tree::create(RelativePath::new("."))?;

    let mut status = Status {
        modified: Vec::new(),
        added: Vec::new(),
        removed: Vec::new(),
    };

    blobs_status(&current_tree, &saved_tree, &mut status);
    tree_status(&current_tree, &saved_tree, &mut status);

    Ok(status)
}

fn blobs_status(current_tree: &Tree, saved_tree: &Tree, status: &mut Status) {
    for current_blob in &current_tree.blobs {
        let path = &current_blob.path;
        let saved_blob = &saved_tree.blobs.iter().find(|b| &b.path == path);
        if let Some(saved_blob) = saved_blob {
            if current_blob.object.digest() != saved_blob.object.digest() {
                status.modified.push(path.clone());
            }
        } else {
            status.added.push(path.clone());
        }
    }

    for saved_blob in &saved_tree.blobs {
        let path = &saved_blob.path;
        let current_blob = current_tree.blobs.iter().find(|b| &b.path == path);
        if current_blob.is_none() {
            status.removed.push(path.clone());
        }
    }
}

fn tree_status(current_tree: &Tree, saved_tree: &Tree, status: &mut Status) {
    for current_tree in &current_tree.trees {
        let path = &current_tree.path;
        let saved_tree = &saved_tree.trees.iter().find(|t| &t.path == path);
        if let Some(saved_tree) = saved_tree {
            blobs_status(&current_tree.object, &saved_tree.object, status);
            tree_status(&current_tree.object, &saved_tree.object, status);
        } else {
            for blob in &current_tree.object.blobs {
                status.added.push(blob.path.clone());
            }
            for tree in &current_tree.object.trees {
                status.added.push(tree.path.clone());
            }
        }
    }
    for saved_tree in &saved_tree.trees {
        let path = &saved_tree.path;
        let current_tree = current_tree.trees.iter().find(|t| &t.path == path);
        if current_tree.is_none() {
            for blob in &saved_tree.object.blobs {
                status.removed.push(blob.path.clone());
            }
            for tree in &saved_tree.object.trees {
                status.removed.push(tree.path.clone());
            }
        }
    }
}
