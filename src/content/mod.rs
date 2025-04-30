use std::{collections::HashMap, path::PathBuf};

use page::Page;

pub mod frontmatter;
pub mod markdown;
pub mod page;

pub fn read_files(root: &PathBuf) -> anyhow::Result<HashMap<PathBuf, Page>> {
    let mut map = HashMap::new();
    for entry in walkdir::WalkDir::new(root) {
        let entry = entry?;

        if entry.file_type().is_file() && entry.path().extension().map(|ext| ext == "md").unwrap_or(false) {
            let path = entry.path().to_path_buf();
            let page = Page::read(&path)?;
            map.insert(path.clone(), page);
        }
    }

    Ok(map)
}
