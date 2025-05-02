use super::{frontmatter::Frontmatter, markdown::render_html};
use crate::config::AppConfig;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fs, path::PathBuf};

#[derive(Default, Clone, Debug, Serialize)]
pub struct Page {
    #[serde(flatten)]
    pub frontmatter: Frontmatter,
    pub outline: Vec<PageHeading>,
    #[serde(skip_serializing)]
    pub content: String,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PageHeading {
    pub level: u8,
    pub id: String,
    pub title: String,
    pub children: Vec<PageHeading>,
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
        headings = build_tree(&headings);

        Ok(Page {
            frontmatter,
            content: html,
            outline: headings,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NavItem {
    pub url: String,
    pub title: String,
}

pub fn filename_to_url(filename: &PathBuf, config: &AppConfig) -> String {
    let content_dir = config.folder.join("content");
    let relative_to_root = &pathdiff::diff_paths(filename, content_dir).unwrap_or(filename.to_owned());
    let folder = relative_to_root.parent().unwrap_or(relative_to_root);
    let filename = relative_to_root.file_stem().unwrap(); //we are sure it is a file
    format!("/{}", folder.join(filename).to_string_lossy())
}

fn build_tree(flat: &[PageHeading]) -> Vec<PageHeading> {
    let mut tree = vec![];
    let mut last_item: Option<&mut PageHeading> = None;

    for item in flat {
        if let Some(last) = &mut last_item {
            match item.level.cmp(&last.level) {
                Ordering::Greater => last.children.push(item.clone()),
                _ => tree.push(item.clone()),
            }
        } else {
            tree.push(item.clone());
        }
        last_item = tree.last_mut();
    }
    tree
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
        assert_eq!(page.outline[0].title, "Test Heading");
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
