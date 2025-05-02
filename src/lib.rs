use axum::http::StatusCode;
use config::AppConfig;
use content::page::{NavItem, filename_to_url};
use serde_yaml::Value;
use std::{collections::HashMap, fs, path::Path};
use templates::{TEMPLATES, load_templates};

pub mod cli;
pub mod config;
pub mod content;
pub mod server;
pub mod templates;

pub fn render(markdown: impl AsRef<Path>, config: &AppConfig) -> anyhow::Result<String> {
    render_page(markdown, config)
}

pub fn render_page(filename: impl AsRef<Path>, config: &AppConfig) -> anyhow::Result<String> {
    let parent_dir = filename.as_ref().parent().unwrap_or(&config.folder);
    let mut template = String::from("layout.html");

    let tera = load_templates(config)?;
    let mut context = tera::Context::new();
    context.insert("config", &config.project_config);

    let dir_config = parent_dir.join(format!("{}.yaml", parent_dir.file_name().unwrap().to_string_lossy()));
    if dir_config.exists() {
        let dir_config = fs::read_to_string(&dir_config)?;
        let yml = serde_yaml::from_str::<HashMap<String, Value>>(&dir_config)?;
        for (key, value) in yml {
            if key == "layout" {
                template = if let Value::String(value) = &value {
                    value.to_string()
                } else {
                    template
                };
            }
            context.insert(&key, &value);
        }
    }

    // get library tree
    let nav = config
        .library
        .iter()
        .filter(|(key, _)| key.starts_with(parent_dir.to_string_lossy().into_owned()))
        .map(|(key, value)| NavItem {
            url: filename_to_url(key, config),
            title: value.frontmatter.title.clone().unwrap(),
        })
        .collect::<Vec<_>>();
    context.insert("sitenav", &nav);

    let page = &config.library[&filename.as_ref().to_path_buf()];
    context.insert("page", page);
    context.insert("content", &page.content);

    let rendered = tera.render(&template, &context).unwrap();

    Ok(rendered)
}

pub fn render_error(config: &AppConfig, code: StatusCode) -> Option<String> {
    let mut context = tera::Context::new();
    context.insert("statuscode", &code.as_u16());
    context.insert("message", &code.canonical_reason());
    if let Ok(tera) = load_templates(config) {
        if let Ok(html) = tera.render("error.html", &context) {
            return Some(html);
        }
        return tera.render("__builtins/error.html", &context).ok();
    }
    TEMPLATES.render("__builtins/error.html", &context).ok()
}
