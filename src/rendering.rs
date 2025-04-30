use crate::{config::AppConfig, templates::load_templates};
use std::{collections::HashMap, fs, path::Path};
use toml::Value;

pub fn render(filename: impl AsRef<Path>, config: &AppConfig) -> anyhow::Result<String> {
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

    let page = &config.library[&filename.as_ref().to_path_buf()];
    context.insert("page", page);
    context.insert("content", &page.content);

    let rendered = tera.render(&template, &context).unwrap();

    Ok(rendered)
}
