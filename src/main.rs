use markdown::Options;
use serde_yaml::Value;
use std::{collections::HashMap, fs};
use tera::{Context, Tera};

fn main() {
    let tera = Tera::new("templates/**/*").unwrap();

    let mut context = Context::new();

    let md = fs::read_to_string("content/batch.md").unwrap();

    if let Some(fm) = extract_frontmatter(&md) {
        let yml = serde_yaml::from_str::<HashMap<String, Value>>(fm).unwrap();
        for (key, value) in yml {
            context.insert(key, &value);
        }
    }

    let mut options = Options::gfm();
    options.parse.constructs.frontmatter = true;
    let html = markdown::to_html_with_options(&md, &options).unwrap();

    context.insert("content", &html);

    let rendered = tera.render("layout.html", &context).unwrap();
    fs::write("output.html", rendered).unwrap();
}

fn extract_frontmatter(markdown: &str) -> Option<&str> {
    let mut parts = markdown.splitn(3, "---");
    parts.next()?;
    let frontmatter = parts.next()?;
    Some(frontmatter.trim())
}
