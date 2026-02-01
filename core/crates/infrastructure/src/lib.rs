pub mod config;
pub mod database;
pub mod event_store;
pub mod projections;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    EnvVarError(#[from] dotenvy::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Url(#[from] url::ParseError),
    #[error("{0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("{0}")]
    ConfigError(#[from] config::Error),
    #[cfg(feature = "sea-query-sqlx")]
    #[error("{0}")]
    SeaQuerySQLx(#[from] database::Error),
}
