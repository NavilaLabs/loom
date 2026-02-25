use std::time::Duration;

use loom_infrastructure::config::CONFIG;
use sqlx::any::AnyPoolOptions;
use tracing::info;
use url::Url;

use crate::{
    Error,
    sea_query_sqlx::infrastructure::{DatabaseType, Provider, StateConnected, StateDisconnected},
};

impl<Scope> Provider<Scope, StateDisconnected> {
    pub async fn connect(uri: &Url) -> Result<Provider<Scope, StateConnected>, Error> {
        sqlx::any::install_default_drivers();

        let mut pool = AnyPoolOptions::new();
        let pool_config = CONFIG.get_database().get_pool();
        let max_size = pool_config.get_max_size();
        pool = pool.max_connections(max_size);
        let min_size = pool_config.get_min_size();
        pool = pool.min_connections(min_size);
        let timeout_seconds = pool_config.get_timeout_seconds();
        pool = pool.idle_timeout(Duration::from_secs(timeout_seconds));
        let database_type = match uri.scheme() {
            "postgres" => DatabaseType::Postgres,
            "sqlite" => DatabaseType::Sqlite,
            schema => {
                return Err(
                    crate::sea_query_sqlx::infrastructure::Error::UnsupportedDatabaseType(
                        schema.to_string(),
                    )
                    .into(),
                );
            }
        };
        info!("Configured database pool: {:?}", pool);
        info!("Establishing connection to database at URL: {}", uri);

        Ok(Provider::new(
            StateConnected::new(pool.connect(uri.as_str()).await?),
            database_type,
        ))
    }
}
