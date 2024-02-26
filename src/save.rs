// This module implements the Save command
// To Save a snapshot, it will do the following
// Locate the files that have been changed by:
//      1. Ignoring any files/dirs in .shtignore
//      2. Add any files/dirs not in the HEAD tree
//      3. Remove any files/dirs that are in index but not the working dirs
//      5. Add any files/dirs that have changed from what is in the HEAD tree
//
use crate::{blob, REPO_ROOT};
use anyhow::Result;
use relative_path::RelativePath;
use std::{fs, path::Path};

pub fn save_all(path: &RelativePath) -> Result<()> {
    println!("Path: {:?}", path);
    // todo create tree
    for entry in fs::read_dir(path.to_path(""))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if path != Path::new(REPO_ROOT) {
                println!("Dir Path: {:?}", path);
                save_all(RelativePath::from_path(&path)?)?;
            }
        } else {
            let blob = blob::Blob::create(&path)?;
            blob.write()?;
        }
    }
    Ok(())
}
