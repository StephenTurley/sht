use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use thiserror::Error;

pub mod blob;
pub mod save;

static REPO_ROOT: &str = "./.sht";

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new repository
    Init,
    /// Save all files not explicitly ignored in the current working directory
    Save,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Init) => init(),
        Some(Commands::Save) => save::save_all(&std::env::current_dir()?),
        None => Ok(()),
    }
}

#[derive(Error, Debug)]
enum InitializationErrors {
    #[error("Repository already exists")]
    RepositoryExists,
}

fn init() -> Result<()> {
    if Path::new("/.sht").exists() {
        bail!(InitializationErrors::RepositoryExists)
    }
    fs::create_dir(REPO_ROOT)?;
    fs::File::create("./.shtignore")?;
    Ok(())
}
