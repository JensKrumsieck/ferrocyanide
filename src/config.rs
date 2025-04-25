use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Config {
    pub title: Option<String>,
}
