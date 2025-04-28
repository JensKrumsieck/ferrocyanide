use anyhow::Context;
use comrak::{Anchorizer, Arena, ComrakPlugins, Options, format_html_with_plugins, html, nodes::NodeValue, parse_document, plugins};
use config::AppConfig;
use serde_yaml::Value;
use std::{cmp::Ordering, collections::HashMap};
use tera::Tera;

pub mod cli;
pub mod config;
pub mod server;

pub fn render(markdown: &str, config: &AppConfig) -> anyhow::Result<String> {
    let tera = Tera::new(&format!("{}/templates/**/*", config.folder.to_string_lossy()))?;
    let mut template = String::from("layout.html");
    let mut context = tera::Context::new();

    context.insert("config", &config.project_config);

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
    options.extension.header_ids = Some("h-".to_string());

    let arena = Arena::new();
    let root = parse_document(&arena, markdown, &options);

    let mut headings = vec![];
    for node in root.descendants() {
        if let NodeValue::Text(ref mut text) = node.data.borrow_mut().value {
            *text = text.replace(".md", "/");
        }
        if let NodeValue::FrontMatter(ref mut fm) = node.data.borrow_mut().value {
            *fm = String::new();
        }
        if let NodeValue::Heading(ref heading) = node.data.borrow().value {
            let mut text_content = Vec::with_capacity(20);
            html::collect_text(node, &mut text_content);

            let text = String::from_utf8(text_content).unwrap();
            let mut anchorizer = Anchorizer::new();
            let id = anchorizer.anchorize(text.clone());

            headings.push((heading.level, id, text));
        }
    }

    let mut html = vec![];
    html.extend(render_toc(&headings).into_bytes());

    let syntect_plugin = plugins::syntect::SyntectAdapter::new(Some("InspiredGitHub"));
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&syntect_plugin);

    format_html_with_plugins(root, &options, &mut html, &plugins)?;

    String::from_utf8(html).context("context")
}

fn render_toc(headings: &[(u8, String, String)]) -> String {
    let mut toc = String::new();
    let mut last_level = 0;

    for (level, id, text) in headings {
        match level.cmp(&last_level) {
            Ordering::Greater => {
                toc.push_str(&format!("<ul class=\"toc-level-{}\">\n", level));
            }
            Ordering::Less => {
                toc.push_str("</ul>\n");
            }
            Ordering::Equal => {}
        }
        toc.push_str(&format!("<li><a href=\"#{}\">{}</a></li>\n", id, text));
        last_level = *level;
    }

    while last_level > 0 {
        toc.push_str("</ul>\n");
        last_level -= 1;
    }

    toc
}
