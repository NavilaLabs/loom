use infrastructure::CONFIG;

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
        let mut url = CONFIG.get_database().get_postgres_admin_uri()?;
        url.set_path(database);

        let pool = sqlx::PgPool::connect(url.as_str()).await?;

        Ok(pool)
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
