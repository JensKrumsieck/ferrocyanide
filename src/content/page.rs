use super::{frontmatter::Frontmatter, markdown::render_html};
use std::{fs, path::PathBuf};

#[derive(Default, Clone, Debug)]
pub struct Page {
    pub frontmatter: Frontmatter,
    pub content: String,
    pub outline: Vec<PageHeadings>,
}

#[derive(Default, Clone, Debug)]
pub struct PageHeadings {
    pub level: u8,
    pub id: String,
    pub text: String,
}

impl Page {
    pub fn read(path: &PathBuf) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        Page::from_string(&content)
    }

    pub fn from_string(content: &str) -> anyhow::Result<Self> {
        let mut headings = Vec::new();
        let mut frontmatter = Frontmatter::read(content).unwrap_or_default();

        let html = render_html(content, &mut headings, &mut frontmatter)?;

        Ok(Page {
            frontmatter,
            content: html,
            outline: headings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_from_string() {
        let content = r#"---
title: "Test Title"
description: "Test Description"
---

# Test Heading
Hello, world
"#;

        let page = Page::from_string(content).unwrap();
        assert_eq!(page.frontmatter.title, Some("Test Title".to_string()));
        assert_eq!(page.frontmatter.description, Some("Test Description".to_string()));
        assert_eq!(page.outline.len(), 1);
        assert_eq!(page.outline[0].level, 1);
        assert_eq!(page.outline[0].text, "Test Heading");
        assert_eq!(page.outline[0].id, "test-heading");
    }

    #[test]
    fn test_page_auto_title() {
        let content = r#"---
description: "Test Description"
---

# Test Heading
Hello, world
"#;

        let page = Page::from_string(content).unwrap();
        assert_eq!(page.frontmatter.title, Some("Test Heading".to_string()));
        assert_eq!(page.frontmatter.description, Some("Test Description".to_string()));
    }
}
