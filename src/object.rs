pub mod blob;
pub mod save;
pub mod tree;

use std::{fs, io::Write};

use anyhow::Result;

use crate::REPO_ROOT;

pub trait Object {
    fn digest(&self) -> &str;
    fn content(&self) -> &str;
    fn t<'a>(&self) -> &'a str;

    fn write(&self) -> Result<()> {
        // TODO use relative path here?
        let blob_path = std::env::current_dir()?
            .join(REPO_ROOT)
            .join("objects/")
            .join(&self.digest()[0..3]);

        if !blob_path.exists() {
            fs::create_dir_all(&blob_path)?;
        }
        let file_name = blob_path.join(&self.digest()[3..]);

        if !file_name.exists() {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(false)
                .open(file_name)?;

            file.write_all(self.content().as_bytes())?;
        }
        Ok(())
    }
}
