use crate::{read_config, render};
use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

pub fn build(folder: Option<PathBuf>) -> anyhow::Result<()> {
    let out_dir = Path::new("dist");
    fs::remove_dir_all(out_dir).ok(); //ignore if fails
    fs::create_dir(out_dir)?;
    let root = folder.unwrap_or_default();
    let content = root.join("content");
    copy_dir_all(root.join("assets"), out_dir.join("assets"))?;

    let config = read_config(Some(root))?;

    for file in walkdir::WalkDir::new(&content) {
        let file = file?;
        if file.path().extension() == Some(OsStr::new("md")) {
            let contents = render(file.path(), &config)?;
            let filename = file.path().strip_prefix(&content)?;
            let filename = filename.with_extension("html");
            let filename = out_dir.join(filename);
            if let Some(parent) = filename.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(filename, contents)?;
        }
    }

    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
