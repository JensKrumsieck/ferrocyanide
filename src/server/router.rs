use crate::{config::AppConfig, content::COLOR_PICKER_JS, render, render_error, resolve_filename};
use axum::{
    Router,
    extract::{Request, State},
    http::{StatusCode, Uri},
    middleware::{self, Next},
    response::{Html, IntoResponse, Redirect},
    routing::get,
};
use tower_http::{compression::CompressionLayer, decompression::RequestDecompressionLayer, services::ServeDir, trace::TraceLayer};

pub(crate) fn app(config: AppConfig) -> Router {
    axum::Router::new()
        .route("/js/SwitchColorMode.js", get(color_picker))
        .nest_service("/assets", ServeDir::new(config.folder.join("assets")))
        .fallback(handler)
        .with_state(config)
        .layer(TraceLayer::new_for_http().on_failure(()))
        .layer(RequestDecompressionLayer::new())
        .layer(CompressionLayer::new())
        .layer(middleware::from_fn(redirect_index))
}

pub(crate) async fn handler(ctx: State<AppConfig>, uri: Uri) -> impl IntoResponse {
    let filename = resolve_filename(&uri, &ctx.folder);
    if filename.exists() {
        match render(&filename, &ctx) {
            Ok(html) => (StatusCode::OK, Html(html)).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Html("Internal Server Error")).into_response(),
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Html(render_error(&ctx, StatusCode::NOT_FOUND).unwrap_or("Unhandled Error occured".into())),
        )
            .into_response()
    }
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

async fn color_picker() -> impl IntoResponse {
    ([("content-type", "text/javascript")], COLOR_PICKER_JS)
}
