use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Serves the site in development mode")]
    Serve {
        folder: Option<PathBuf>,
    },
    Build {
        folder: Option<PathBuf>,
    },
}
