use anyhow::{bail, Result};
use clap::Parser;
use command::save;
use command::status;
use command::Commands;
use relative_path::RelativePath;
use std::fs;
use std::path::Path;
use thiserror::Error;

pub mod command;
pub mod object;

static REPO_ROOT: &str = "./.sht";

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Init) => init(),
        Some(Commands::Save) => {
            save::execute(RelativePath::new("./"))?;
            Ok(())
        }
        Some(Commands::Status) => {
            let status = status::execute()?;
            println!("Added: {:?}", status.added);
            println!("Modified: {:?}", status.modified);
            println!("Removed: {:?}", status.removed);
            Ok(())
        }
        None => Ok(()),
    }
}

#[derive(Error, Debug)]
enum InitializationErrors {
    #[error("Repository already exists")]
    RepositoryExists,
}

fn init() -> Result<()> {
    if Path::new(REPO_ROOT).exists() {
        bail!(InitializationErrors::RepositoryExists)
    }
    fs::create_dir(REPO_ROOT)?;
    fs::File::create(".shtignore")?;
    Ok(())
}
