use clap::Subcommand;

pub mod save;

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new repository
    Init,
    /// Save all files not explicitly ignored in the current working directory
    Save,
    /// Show files that have changed since the last save
    Status,
}
