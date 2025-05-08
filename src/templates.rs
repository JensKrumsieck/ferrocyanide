use crate::config::AppConfig;
use once_cell::sync::Lazy;
use tera::Tera;

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

    Ok(tera)
}
