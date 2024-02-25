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
use std::{fs, path::PathBuf};

pub fn save_all(path: &PathBuf) -> Result<()> {
    // todo create tree
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if path != std::env::current_dir()?.join(REPO_ROOT) {
                save_all(&path)?;
            }
        } else {
            let blob = blob::create(&path)?;
            blob.write()?;
        }
    }
    Ok(())
}
