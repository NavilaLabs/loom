use std::sync::LazyLock;

use dotenvy::var;
use serde::{Deserialize, Serialize};

mod application;
mod database;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("YAML deserialization error: {0}")]
    YamlDeserializationError(#[from] serde_yaml::Error),
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] dotenvy::Error),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

type Result<T> = std::result::Result<T, Error>;

pub static CONFIG: LazyLock<Config> =
    LazyLock::new(|| load_config().expect("Failed to load configuration"));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub application: application::Application,
    pub database: database::Database,
}

impl Config {
    #[must_use]
    pub fn get_application(&self) -> &application::Application {
        &self.application
    }

    #[must_use]
    pub fn get_database(&self) -> &database::Database {
        &self.database
    }
}

pub fn load_config() -> Result<Config> {
    let project_root = var("APP_PROJECT_ROOT")?;
    let config_path = format!("{project_root}/config");

    let mut file_string = String::new();
    let application_config_path = format!("{config_path}/application.yaml");
    let database_config_path = format!("{config_path}/database.yaml");
    let logging_config_path = format!("{config_path}/logging.yaml");
    file_string.push_str(&std::fs::read_to_string(&application_config_path)?);
    file_string.push('\n');
    file_string.push_str(&std::fs::read_to_string(&database_config_path)?);
    file_string.push('\n');
    file_string.push_str(&std::fs::read_to_string(&logging_config_path)?);

    let mut source = serde_vars::EnvSource::default();
    let de = serde_yaml::Deserializer::from_str(&file_string);
    let config: Config = serde_vars::deserialize(de, &mut source)?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use dotenvy::dotenv;

    use super::*;

    #[test]
    fn test_load_config() {
        dotenv().ok();
        let config = load_config().unwrap();
        assert_eq!(config.get_application().get_name(), "Gyst");
        assert_eq!(config.get_database().get_uri().get_port(), 5432);
    }
}
