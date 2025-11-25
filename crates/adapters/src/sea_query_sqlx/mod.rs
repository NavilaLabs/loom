use std::time::Duration;

use infrastructure::CONFIG;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] infrastructure::config::Error),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

pub struct Database;

#[async_trait::async_trait]
impl infrastructure::Database<sqlx::PgPool> for Database {
    type Error = Error;

    async fn establish_connection(&self, database: &str) -> Result<sqlx::PgPool, Self::Error> {
        let database_config = CONFIG.get_database();
        let mut url = database_config.get_postgres_admin_uri()?;
        url.set_path(database);

        let mut pool = PgPoolOptions::new();
        if let Some(pool_config) = database_config.get_pool() {
            if let Some(max_size) = pool_config.get_max_size() {
                pool = pool.max_connections(max_size);
            }
            if let Some(min_size) = pool_config.get_min_size() {
                pool = pool.min_connections(min_size);
            }
            if let Some(timeout_seconds) = pool_config.get_timeout_seconds() {
                pool = pool.idle_timeout(Duration::from_secs(timeout_seconds));
            }
        }

        Ok(pool.connect(url.as_str()).await?)
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::{CONFIG, Database};

    #[tokio::test]
    async fn test_establish_admin_connection() {
        let db = super::Database;

        let admin_pool = db.establish_admin_connection().await.unwrap();
    }
}
