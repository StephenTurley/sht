// This module will implement the Save command
// To Save a snapshot, it will do the following
// Locate the files that have been changed by:
//      1. Ignore any files/dirs in .shtignore
//      2. Add any files/dirs not in the index
//      3. Remove any files/dirs that are in index but not the working dirs
//      5. Add any files/dirs that have changed from what is in the index (compare timestamp and hash)
//
use crate::blob;
use anyhow::Result;
use std::{fs, path::PathBuf};

pub fn save_all(path: PathBuf) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            // create_tree
        } else {
            blob::create(&path)?;
        }
    }
    Ok(())
}



