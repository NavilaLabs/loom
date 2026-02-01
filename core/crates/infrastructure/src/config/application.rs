use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "application")]
pub struct Application {
    environment: String,
    name: String,
    project_root: String,
}

impl Application {
    pub fn get_environment(&self) -> &str {
        &self.environment
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_project_root(&self) -> &str {
        &self.project_root
    }
}
