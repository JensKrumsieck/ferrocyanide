pub(crate) mod router;

use crate::config::Config;
use router::handler;
use std::{fs, path::PathBuf};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Default, Clone, Debug)]
pub struct AppConfig {
    pub folder: PathBuf,
    pub config: Config,
}

pub async fn serve(folder: Option<PathBuf>) -> anyhow::Result<()> {
    let folder = folder.unwrap_or(PathBuf::from("content"));
    
    let cfg = if folder.join("ferrocyanide.yaml").exists() {
        let config_file = fs::read_to_string(folder.join("ferrocyanide.yaml"))?;
        serde_yaml::from_str::<Config>(&config_file).unwrap_or_default()
    } else {
        Config::default()
    };

    let config = AppConfig {
        folder,
        config: cfg,
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = axum::Router::new()
        .nest_service("/assets", ServeDir::new(config.folder.join("assets")))
        .fallback(handler)
        .with_state(config)
        .layer(TraceLayer::new_for_http().on_failure(()));

    let listener = TcpListener::bind("0.0.0.0:8192").await?;

    info!("Listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
