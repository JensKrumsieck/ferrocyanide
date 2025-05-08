use std::{collections::HashMap, ops::Deref};

use crate::{CONTEXT, Context, config::AppConfig};
use once_cell::sync::Lazy;
use tera::{Tera, Value};

pub static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let mut tera = Tera::default();

    tera.add_raw_templates(vec![
        ("__builtins/error.html", include_str!("builtins/error.html")),
        ("__builtins/toc-item.html", include_str!("builtins/toc-item.html")),
        ("__builtins/toc.html", include_str!("builtins/toc.html")),
        ("__builtins/nav.html", include_str!("builtins/nav.html")),
        ("__builtins/theme_switch.html", include_str!("builtins/theme_switch.html")),
    ])
    .unwrap();
    tera
});

pub fn load_templates(config: &AppConfig) -> anyhow::Result<Tera> {
    let mut tera = Tera::parse(&format!("{}/templates/**/*", config.folder.to_string_lossy()))?;
    tera.extend(&TEMPLATES)?;
    tera.build_inheritance_chains()?;

    let prefix = if let Some(meta) = &config.project_config.project {
        meta.root_dir.clone().unwrap_or_default()
    } else {
        String::new()
    };

    tera.register_filter("url", move |value: &Value, _: &HashMap<String, Value>| -> tera::Result<Value> {
        let path = value.as_str().ok_or("Expected a string for path")?;

        let ctx_guard = CONTEXT.read().unwrap();
        let ctx = ctx_guard.deref();

        let full_url = if ctx == &Context::Build {
            format!("{prefix}{path}")
        } else {
            path.to_string()
        };
        Ok(Value::String(full_url))
    });

    Ok(tera)
}
