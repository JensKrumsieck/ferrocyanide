pub(crate) mod router;

use crate::config::{AppConfig, ProjectConfig, get_config_path};
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Redirect},
};
use router::handler;
use std::{fs, path::PathBuf};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn serve(folder: Option<PathBuf>) -> anyhow::Result<()> {
    let folder = folder.unwrap_or(PathBuf::from("content"));
    let config_file = get_config_path(&folder);

    let project_config = if config_file.exists() {
        let config_file = fs::read_to_string(config_file)?;
        toml::from_str::<ProjectConfig>(&config_file).unwrap_or_default()
    } else {
        ProjectConfig::default()
    };

    let config = AppConfig { folder, project_config };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,tower_http=debug,axum::rejection=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = axum::Router::new()
        .nest_service("/assets", ServeDir::new(config.folder.join("assets")))
        .fallback(handler)
        .with_state(config)
        .layer(TraceLayer::new_for_http().on_failure(()))
        .layer(middleware::from_fn(redirect_index));

    let listener = TcpListener::bind("0.0.0.0:8192").await?;

    info!("Listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn redirect_index(req: Request, next: Next) -> Result<impl IntoResponse, StatusCode> {
    let path = req.uri().path();
    let path = path.strip_suffix("/").unwrap_or(path);
    if let Some(stripped) = path.strip_suffix("/index") {
        let new_location = if stripped.is_empty() {
            "/".to_string()
        } else {
            format!("{}/", stripped.trim_end_matches('/'))
        };
        return Ok(Redirect::permanent(&new_location).into_response());
    }

    Ok(next.run(req).await)
}
