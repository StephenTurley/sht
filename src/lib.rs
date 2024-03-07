use thiserror::Error;

pub mod command;
pub mod object;

static REPO_ROOT: &str = "./.sht";

#[derive(Error, Debug)]
enum InitializationErrors {
    #[error("Repository already exists")]
    RepositoryExists,
}
