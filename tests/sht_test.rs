use relative_path::RelativePath;
use sht::command::init;
use tempfile::tempdir;

#[test]
fn it_works() {
    let test_repo_path = tempdir().unwrap();
    std::env::set_current_dir(&test_repo_path).unwrap();

    init::execute(RelativePath::new("./")).unwrap();

    assert!(test_repo_path.path().join(".sht").exists());
}
