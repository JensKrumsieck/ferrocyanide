use crate::render;

use super::AppConfig;
use axum::{
    extract::State,
    http::{StatusCode, Uri},
    response::Html,
};
use std::fs;

pub(crate) async fn handler(ctx: State<AppConfig>, uri: Uri) -> Result<Html<String>, StatusCode> {
    let path = uri.path();
    let path = path.trim_start_matches('/').trim_end_matches('/');
    let path = if path.is_empty() { "index" } else { path };

    let content_dir = ctx.folder.join("content");
    let filename = content_dir.join(format!("{path}.md"));

    if filename.exists() {
        let content = fs::read_to_string(filename).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let html = render(&content, &ctx).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Html(html))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
