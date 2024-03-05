pub mod blob;
pub mod save;
pub mod tree;

use std::{fs, io::Write, path::PathBuf};

use anyhow::Result;
use relative_path::RelativePath;

use crate::REPO_ROOT;

pub trait Object {
    fn digest(&self) -> &str;
    fn content(&self) -> &str;
    fn t<'a>(&self) -> &'a str;

    fn write(&self) -> Result<()> {
        let object_path = ObjectPath::from(self.digest());

        if !object_path.path.exists() {
            fs::create_dir_all(&object_path.path)?;
        }

        if !object_path.name.exists() {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(false)
                .open(object_path.name)?;

            file.write_all(self.content().as_bytes())?;
        }
        Ok(())
    }
}

struct ObjectPath {
    path: PathBuf,
    name: PathBuf,
}

impl ObjectPath {
    pub fn from(hash: &str) -> ObjectPath {
        let path = RelativePath::new("objects/")
            .join(&hash[0..3])
            .to_path(REPO_ROOT);

        let name = path.join(&hash[3..]);

        ObjectPath { path, name }
    }
}
