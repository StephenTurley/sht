use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use relative_path::RelativePath;
use sht::command::{init, save};
use tempfile::tempdir;

#[test]
fn it_works() {
    let test_repo_path = tempdir().unwrap();
    std::env::set_current_dir(&test_repo_path).unwrap();

    init::execute(RelativePath::new("./")).unwrap();

    assert!(test_repo_path.path().join(".sht").exists());
}

#[test]
fn tracks_repo_status() {
    let test_repo_path = tempdir().unwrap();
    let mut modify_me = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(test_repo_path.path().join("file1.txt"))
        .unwrap();

    // init
    std::env::set_current_dir(&test_repo_path).unwrap();
    init::execute(RelativePath::new("./")).unwrap();
    modify_me.write_all(b"content").unwrap();
    save::execute(RelativePath::new("./")).unwrap();

    //modify and save
    modify_me.write_all(b"new content").unwrap();
    let status = sht::command::status::execute().unwrap();
    assert_eq!(status.modified[0], "./file1.txt");
    assert!(status.added.is_empty());
    assert!(status.removed.is_empty());
    save::execute(RelativePath::new("./")).unwrap();

    // add and save
    let new_file = test_repo_path.path().join("file2.txt");
    fs::write(new_file, "content").unwrap();
    let status = sht::command::status::execute().unwrap();
    assert_eq!(status.added[0], "./file2.txt");
    assert!(status.modified.is_empty());
    assert!(status.removed.is_empty());
    save::execute(RelativePath::new("./")).unwrap();

    // remove and save
    fs::remove_file(test_repo_path.path().join("file2.txt")).unwrap();
    let status = sht::command::status::execute().unwrap();
    assert_eq!(status.removed[0], "./file2.txt");
    assert!(status.modified.is_empty());
    assert!(status.added.is_empty());
}
