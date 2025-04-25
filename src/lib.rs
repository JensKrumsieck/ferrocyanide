use anyhow::Context;
use comrak::{
    Arena, ComrakPlugins, Options, format_html_with_plugins, nodes::NodeValue, parse_document,
    plugins,
};
use config::Config;
use serde_yaml::Value;
use std::collections::HashMap;
use tera::Tera;

pub mod cli;
pub mod config;
pub mod server;

pub fn render(markdown: &str, config: &Config) -> anyhow::Result<String> {
    let tera = Tera::new("templates/**/*")?;
    let mut template = String::from("layout.html");
    let mut context = tera::Context::new();

    context.insert("config", &config);

    if let Some(fm) = extract_frontmatter(markdown) {
        let yml = serde_yaml::from_str::<HashMap<String, Value>>(fm)?;
        for (key, value) in yml {
            context.insert(&key, &value);
            if key == "layout" {
                template = serde_yaml::to_string(&value)?;
            }
        }
    }

    let html = render_markdown(markdown)?;
    context.insert("content", &html);

    let rendered = tera.render(&template, &context).unwrap();

    Ok(rendered)
}

fn extract_frontmatter(markdown: &str) -> Option<&str> {
    let mut parts = markdown.splitn(3, "---");
    parts.next()?;
    let frontmatter = parts.next()?;
    Some(frontmatter.trim())
}

fn render_markdown(markdown: &str) -> anyhow::Result<String> {
    let mut options = Options::default();
    options.extension.front_matter_delimiter = Some("---".to_string());

    let arena = Arena::new();
    let root = parse_document(&arena, markdown, &options);

    for node in root.descendants() {
        if let NodeValue::Text(ref mut text) = node.data.borrow_mut().value {
            *text = text.replace(".md", "/");
        }
        if let NodeValue::FrontMatter(ref mut fm) = node.data.borrow_mut().value {
            *fm = String::new();
        }
    }

    let mut html = vec![];

    let syntect_plugin = plugins::syntect::SyntectAdapter::new(Some("InspiredGitHub"));
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&syntect_plugin);

    format_html_with_plugins(root, &options, &mut html, &plugins)?;

    String::from_utf8(html).context("context")
}
