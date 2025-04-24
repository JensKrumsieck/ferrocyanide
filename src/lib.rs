use std::collections::HashMap;

use markdown::Options;
use serde_yaml::Value;
use tera::{Context, Tera};

pub mod cli;
pub mod server;

pub fn render(markdown: &str) -> anyhow::Result<String> {
    let tera = Tera::new("templates/**/*")?;
    let mut template = String::from("layout.html");
    let mut context = Context::new();

    if let Some(fm) = extract_frontmatter(markdown) {
        let yml = serde_yaml::from_str::<HashMap<String, Value>>(fm)?;
        for (key, value) in yml {
            context.insert(&key, &value);
            if key == "layout" {
                template = serde_yaml::to_string(&value)?;
            }
        }
    }

    let mut options = Options::gfm();
    options.parse.constructs.frontmatter = true;
    let html = markdown::to_html_with_options(markdown, &options).unwrap();

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
