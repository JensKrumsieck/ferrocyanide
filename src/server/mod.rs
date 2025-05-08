pub(crate) mod router;
use crate::{read_config, Context, CONTEXT};
use router::app;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn serve(folder: Option<PathBuf>) -> anyhow::Result<()> {
    *CONTEXT.write().unwrap() = Context::Serve;
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
