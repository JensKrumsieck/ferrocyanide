use config::AppConfig;
use std::path::Path;

pub mod cli;
pub mod config;
pub mod rendering;
pub mod server;
pub mod templates;

pub fn render(markdown: impl AsRef<Path>, config: &AppConfig) -> anyhow::Result<String> {
    rendering::render(markdown, config)
}
