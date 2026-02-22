#[cfg(feature = "sea-query-sqlx")]
mod sea_query_sqlx;
#[cfg(feature = "sea-query-sqlx")]
pub use sea_query_sqlx::*;

extern crate infrastructure as infra;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    DateTimeError(#[from] chrono::ParseError),
    #[error("{0}")]
    ModulesError(#[from] modules::Error),
    #[error("{0}")]
    JsonError(#[from] serde_json::Error),
    #[error("{0}")]
    InfrastructureError(#[from] infra::Error),
    #[error("{0}")]
    InfrastructureImplError(#[from] sea_query_sqlx::infrastructure::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Url(#[from] url::ParseError),
    #[cfg(feature = "sea-query-sqlx")]
    #[error("{0}")]
    SeaQuerySqlxError(#[from] sea_query_sqlx::Error),
}
