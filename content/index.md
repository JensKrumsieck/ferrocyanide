# Welcome to Ferrocyanide
Ferrocyanide is a small `proof-of-concept` Static-Site-Generator written in ![Rust][rust-image] 

[rust-image]: https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white

Here is some code for your enjoyment:
```rust
use clap::Parser;
use ferrocyanide::{
    cli::{Cli, Commands},
    server,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Serve { folder } => server::serve(folder).await,
    }
}

```