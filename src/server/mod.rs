pub(crate) mod router;

use axum::handler::Handler;
use bon::Builder;
use router::handler;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Default, Builder, Clone, Debug)]
pub struct Config {
    pub folder: PathBuf,
}

pub async fn serve(folder: Option<PathBuf>) -> anyhow::Result<()> {
    let config = Config::builder()
        .folder(folder.unwrap_or(PathBuf::from("content")))
        .build();

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
        .fallback_service(
            ServeDir::new("static").not_found_service(Handler::with_state(handler, config)),
        )
        .layer(TraceLayer::new_for_http().on_failure(()));

    let listener = TcpListener::bind("0.0.0.0:8192").await?;

    info!("Listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
