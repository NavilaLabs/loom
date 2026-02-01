use admin_migrations::MigratorTrait;
use async_trait::async_trait;

use crate::{
    Error,
    database::{
        Pool,
        migrate::Migrate,
        sea_query_sqlx::{ScopeAdmin, ScopeTenant, StateConnected},
    },
};

#[async_trait]
impl Migrate for Pool<ScopeAdmin, StateConnected> {
    type Error = Error;

    async fn migrate_database(&self) -> Result<(), <Self as Migrate>::Error> {
        let uri = self.as_ref().connect_options().database_url.clone();
        let pool = sea_orm::Database::connect(uri.as_str()).await?;

        admin_migrations::Migrator::up(&pool, None).await?;

        Ok(())
    }
}

#[async_trait]
impl Migrate for Pool<ScopeTenant, StateConnected> {
    type Error = Error;

    async fn migrate_database(&self) -> Result<(), <Self as Migrate>::Error> {
        let uri = self.as_ref().connect_options().database_url.clone();
        let pool = sea_orm::Database::connect(uri.as_str()).await?;

        tenant_migrations::Migrator::up(&pool, None).await?;

        Ok(())
    }
}
