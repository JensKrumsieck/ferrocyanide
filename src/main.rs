use clap::Parser;
use ferrocyanide::{
    build,
    cli::{Cli, Commands},
    server,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Serve { folder } => server::serve(folder).await,
        Commands::Build { folder } => build::build(folder),
    }
}
