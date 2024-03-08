use std::fs;

use anyhow::Result;
use relative_path::RelativePath;

use crate::{
    object::{save::Save, Object},
    REPO_ROOT,
};

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
    new_save.write()?;
    fs::write(head_path, new_save.digest())?;
    Ok(())
}
