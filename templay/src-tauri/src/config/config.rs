use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigTemplate {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigExternalEditor {
    pub name: String,
    pub command: String,
    pub args: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub version: u32,
    pub external_editor: Option<ConfigExternalEditor>,
    pub templates: Vec<ConfigTemplate>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 0,
            external_editor: None,
            templates: vec![],
        }
    }
}
