use std::fmt::Display;

use embassy_futures::join::join;
use log::info;

use crate::config::CONFIG;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database is already initialized")]
    AlreadyInitialized,
    #[error("Failed to initialize database")]
    FailedToInitialize,
}

#[async_trait::async_trait]
pub trait Database<T>
where
    T: Send + Sync,
{
    const DEFAULT_DATABASE_NAME: &'static str;
    type Error: Display + Send + Sync + From<Error>;

    async fn establish_admin_connection(&self) -> Result<T, Self::Error> {
        info!("Establishing connection to admin database");
        self.establish_connection(CONFIG.get_database().get_databases().get_admin().get_name())
            .await
    }

    async fn establish_tenant_connection(&self, tenant_token: &str) -> Result<T, Self::Error> {
        let tenant_db = format!(
            "{}{}",
            CONFIG
                .get_database()
                .get_databases()
                .get_tenant()
                .get_name_prefix(),
            tenant_token
        );
        info!("Establishing connection to tenant database: {}", tenant_db);

        self.establish_connection(&tenant_db).await
    }

    async fn establish_default_connection(&self) -> Result<T, Self::Error> {
        self.establish_connection(Self::DEFAULT_DATABASE_NAME).await
    }

    async fn establish_connection(&self, database: &str) -> Result<T, Self::Error>;

    async fn close_connection(&self, pool: T);
}

#[async_trait::async_trait]
pub trait Initialize<Pool, T>: Database<Pool>
where
    Pool: Send + Sync,
    T: Database<Pool> + Send + Sync,
{
    async fn is_initialized(&self, database: &T) -> bool {
        database.establish_admin_connection().await.is_ok()
    }

    async fn initialize_database(&self, database: &T) -> Result<(), Self::Error> {
        let (is_initialized, pool) = join(
            self.is_initialized(database),
            database.establish_default_connection(),
        )
        .await;

        match (is_initialized, pool) {
            (false, Ok(pool)) => {
            self.create_admin_database(&pool).await?;
            self.create_tenant_database_template(&pool).await?;
            self.close_connection(pool).await;
            Ok(())
            }
            (true, _) => Err(Error::AlreadyInitialized.into()),
            (false, Err(_)) => Err(Error::FailedToInitialize.into()),
        }
    }

    async fn create_admin_database(&self, pool: &Pool) -> Result<(), Self::Error>;

    async fn create_tenant_database_template(&self, pool: &Pool) -> Result<(), Self::Error>;
}

#[async_trait::async_trait]
pub trait Migrate<Pool, T>: MigrateAdmin<Pool, T> + MigrateTenant<Pool, T>
where
    Pool: Send + Sync,
    T: Database<Pool> + Send + Sync,
{
    async fn migrate_database(&self, pool: &T) -> Result<(), Self::Error> {
        self.run_admin_migrations(pool).await?;
        self.run_tenant_migrations(pool).await
    }
}

#[async_trait::async_trait]
pub trait MigrateAdmin<Pool, T>: Database<Pool>
where
    Pool: Send + Sync,
    T: Database<Pool> + Send + Sync,
{
    async fn run_admin_migrations(&self, pool: &T) -> Result<(), Self::Error>;
}

#[async_trait::async_trait]
pub trait MigrateTenant<Pool, T>: Database<Pool>
where
    Pool: Send + Sync,
    T: Database<Pool> + Send + Sync,
{
    async fn run_tenant_migrations(&self, pool: &T) -> Result<(), Self::Error>;
}
