pub(crate) mod router;
use crate::{
    config::{AppConfig, ProjectConfig, get_config_path},
    content,
};
use router::app;
use std::{fs, path::PathBuf};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn serve(folder: Option<PathBuf>) -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,tower_http=debug,axum::rejection=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = TcpListener::bind("0.0.0.0:8192").await?;
    let config = read_config(folder)?;

    info!("Listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app(config)).await?;
    Ok(())
}

fn read_config(folder: Option<PathBuf>) -> anyhow::Result<AppConfig> {
    let folder = folder.unwrap_or(PathBuf::from("content"));
    let config_file = get_config_path(&folder);

    let project_config = if config_file.exists() {
        let config_file = fs::read_to_string(&config_file)?;
        serde_yaml::from_str::<ProjectConfig>(&config_file).unwrap()
    } else {
        ProjectConfig::default()
    };
    let library = content::read_files(&folder)?;

    Ok(AppConfig {
        folder,
        project_config,
        library,
    })
}
