use std::fmt::Display;

use crate::config::CONFIG;

#[async_trait::async_trait]
pub trait Database<T>
where
    T: Send + Sync,
{
    type Error: Display + Send + Sync;

    async fn establish_admin_connection(&self) -> Result<T, Self::Error> {
        self.establish_connection(CONFIG.get_database().get_databases().get_admin().get_name())
            .await
    }

    async fn establish_tenant_connection(&self, tenant_token: &str) -> Result<T, Self::Error> {
        let tenant_db = format!(
            "{}{}",
            CONFIG.get_database().get_databases().get_admin().get_name(),
            tenant_token
        );

        self.establish_connection(&tenant_db).await
    }

    async fn establish_uninitialized_connection(&self) -> Result<T, Self::Error> {
        self.establish_connection("postgres").await
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
    async fn initialize_database(&self, database: &T) -> Result<(), Self::Error> {
        if let Ok(pool) = database.establish_uninitialized_connection().await {
            self.create_admin_database(&pool).await?;
            self.create_tenant_database_template(&pool).await?;
            self.close_connection(pool).await;
            if let Ok(admin_pool) = database.establish_admin_connection().await {
                self.drop_default_database(&admin_pool).await?;
            }
        }

        Ok(())
    }

    async fn create_admin_database(&self, pool: &Pool) -> Result<(), Self::Error>;

    async fn create_tenant_database_template(&self, pool: &Pool) -> Result<(), Self::Error>;

    async fn drop_default_database(&self, pool: &Pool) -> Result<(), Self::Error>;
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
