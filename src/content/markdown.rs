use super::{frontmatter::Frontmatter, page::PageHeading};
use comrak::{
    Anchorizer, Arena, ComrakPlugins, Options, format_html_with_plugins, html,
    nodes::{AstNode, NodeValue},
    parse_document, plugins,
};

pub fn render_html(content: &str, headings: &mut Vec<PageHeading>, frontmatter: &mut Frontmatter) -> anyhow::Result<String> {
    //TODO: Global Options based on config
    let mut options = Options::default();
    options.extension.front_matter_delimiter = Some("---".to_string());
    options.extension.header_ids = Some("h-".to_string());

    let arena = Arena::new();
    let root = parse_document(&arena, content, &options);

    //sanitze the content
    for node in root.descendants() {
        if let NodeValue::Text(ref mut text) = node.data.borrow_mut().value {
            *text = text.replace(".md", "/");
        }
        if let NodeValue::FrontMatter(ref mut fm) = node.data.borrow_mut().value {
            *fm = String::new();
        }
    }

    extract_headings(root, headings);

    // get title
    if frontmatter.title.is_none() {
        frontmatter.title = Some(get_document_title(content)?);
    }

    let syntect_plugin = plugins::syntect::SyntectAdapter::new(Some("InspiredGitHub"));
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&syntect_plugin);

    let mut html = vec![];
    format_html_with_plugins(root, &options, &mut html, &plugins)?;

    String::from_utf8(html).map_err(|e| anyhow::anyhow!("Failed to convert HTML to UTF-8: {}", e))
}

fn extract_headings<'a>(root: &'a AstNode<'a>, headings: &mut Vec<PageHeading>) {
    for node in root.descendants() {
        if let NodeValue::Heading(ref heading) = node.data.borrow().value {
            let mut text_content = Vec::with_capacity(30);
            html::collect_text(node, &mut text_content);

            let text = String::from_utf8(text_content).unwrap();
            let mut anchorizer = Anchorizer::new();
            let id = anchorizer.anchorize(text.clone());

            headings.push(PageHeading {
                level: heading.level,
                id,
                title: text,
                children: vec![]
            });
        }
    }
}

fn get_document_title(document: &str) -> anyhow::Result<String> {
    let arena = Arena::new();
    let root = parse_document(&arena, document, &Options::default());

    for node in root.children() {
        let header = match node.data.clone().into_inner().value {
            NodeValue::Heading(c) => c,
            _ => continue,
        };

        if header.level != 1 {
            continue;
        }

        let mut text = Vec::with_capacity(30);
        html::collect_text(node, &mut text);
        return Ok(String::from_utf8(text)?);
    }

    Ok("Untitled Document".to_string())
}
