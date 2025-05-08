use crate::content::page::Page;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub fn get_config_path(project_dir: impl AsRef<Path>) -> PathBuf {
    project_dir.as_ref().join("config.yaml")
}

#[derive(Default, Clone, Debug)]
pub struct AppConfig {
    pub folder: PathBuf,
    pub library: HashMap<PathBuf, Page>,
    pub project_config: ProjectConfig,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ProjectConfig {
    pub project: Option<ProjectMetadata>,
    pub nav: Option<Vec<NavItem>>,
    //TODO: Markdown parser cfg
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ProjectMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "rootDir")]
    pub root_dir: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct NavItem {
    #[serde(flatten)]
    pub item: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_path() {
        let project_dir = PathBuf::from("/path/to/project");
        let config_path = get_config_path(&project_dir);
        assert_eq!(config_path, PathBuf::from("/path/to/project/config.yaml"));
    }

    #[test]
    fn test_read_config() {
        let config = r#"
project:
    title: Ferrocyanide
    description: A project about Ferrocyanide
"#;
        let config: ProjectConfig = serde_yaml::from_str(config).unwrap();

        if let Some(project) = &config.project {
            assert_eq!(project.title, Some("Ferrocyanide".to_string()));
            assert_eq!(project.description, Some("A project about Ferrocyanide".to_string()));
        } else {
            panic!("Project config should be present");
        }
    }
}
