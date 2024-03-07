use anyhow::Result;
use clap::Parser;
use relative_path::RelativePath;
use sht::command::{init, save, status, Commands};

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
        Some(Commands::Init) => {
            init::execute(RelativePath::new("./"))?;
            Ok(())
        }
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
