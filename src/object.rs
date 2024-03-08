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
    fn write(&self) -> Result<()>;
}
pub fn write_object(object: &impl Object) -> Result<()> {
    let object_path = ObjectPath::from(object.digest());

    if !object_path.directory.exists() {
        fs::create_dir_all(&object_path.directory)?;
    }

    if !object_path.file_name.exists() {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .open(object_path.file_name)?;

        file.write_all(object.content().as_bytes())?;
    }
    Ok(())
}

struct ObjectPath {
    directory: PathBuf,
    file_name: PathBuf,
}

impl ObjectPath {
    pub fn from(hash: &str) -> ObjectPath {
        let directory = RelativePath::new("objects/")
            .join(&hash[0..3])
            .to_path(REPO_ROOT);

        let file_name = directory.join(&hash[3..]);

        ObjectPath {
            directory,
            file_name,
        }
    }
}
