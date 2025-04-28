use crate::config::AppConfig;
use once_cell::sync::Lazy;
use tera::Tera;

pub static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![("_builtins/404.html", include_str!("builtins/404.html"))])
        .unwrap();
    tera
});

pub fn load_templates(config: &AppConfig) -> anyhow::Result<Tera> {
    let mut tera = Tera::parse(&format!("{}/templates/**/*", config.folder.to_string_lossy()))?;
    tera.extend(&TEMPLATES)?;
    tera.build_inheritance_chains()?;

    Ok(tera)
}
