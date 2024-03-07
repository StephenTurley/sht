use std::fs;

use anyhow::{bail, Result};
use relative_path::RelativePath;

use crate::{InitializationErrors, REPO_ROOT};

pub fn execute(path: &RelativePath) -> Result<()> {
    let repo_path = path.join(REPO_ROOT).to_path("");
    if repo_path.exists() {
        bail!(InitializationErrors::RepositoryExists)
    }
    fs::create_dir(repo_path)?;
    Ok(())
}
