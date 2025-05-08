use axum::http::{StatusCode, Uri};
use config::{AppConfig, ProjectConfig, get_config_path};
use content::page::{NavItem, filename_to_url};
use once_cell::sync::Lazy;
use serde_yaml::Value;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::RwLock,
};
use templates::{TEMPLATES, load_templates};

pub mod build;
pub mod cli;
pub mod config;
pub mod content;
pub mod server;
pub mod templates;

#[derive(PartialEq)]
enum Context {
    Build,
    Serve,
}

static CONTEXT: Lazy<RwLock<Context>> = Lazy::new(|| RwLock::new(Context::Serve));

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
    context.insert("path", &resolve_path(filename.as_ref(), &config.folder));

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

fn read_config(folder: Option<PathBuf>) -> anyhow::Result<AppConfig> {
    let folder = folder.unwrap_or(PathBuf::from("."));
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

fn resolve_filename(uri: &Uri, root_dir: &Path) -> PathBuf {
    let path = uri.path();
    let path = path.trim_start_matches('/').trim_end_matches('/');
    let path = if path.is_empty() { "index" } else { path };

    let content_dir = root_dir.join("content");
    let filename = content_dir.join(format!("{path}.md"));

    if filename.exists() {
        filename
    } else {
        content_dir.join(format!("{path}/index.md"))
    }
}

fn resolve_path(path: &Path, root_dir: &Path) -> String {
    let root_dir = root_dir.join("content");
    let rel = path.strip_prefix(root_dir).unwrap_or(path);

    let as_str = rel.to_string_lossy().to_string();
    let as_str = as_str.strip_suffix(".md").unwrap_or(&as_str);
    let as_str = as_str.strip_suffix("index").unwrap_or(as_str);
    format!("/{}", as_str.strip_suffix("/").unwrap_or(as_str))
}
