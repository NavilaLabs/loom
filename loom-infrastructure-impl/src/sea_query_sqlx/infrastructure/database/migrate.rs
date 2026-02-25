use loom_admin_migrations::MigratorTrait;
use async_trait::async_trait;
use loom_infrastructure::database::Migrate;

use crate::{
    Error,
    sea_query_sqlx::infrastructure::{Provider, ScopeAdmin, ScopeTenant, StateConnected},
};

#[async_trait]
impl Migrate for Provider<ScopeAdmin, StateConnected> {
    type Error = Error;

    async fn migrate_database(&self) -> Result<(), <Self as Migrate>::Error> {
        let uri = self.as_ref().connect_options().database_url.clone();
        let pool = sea_orm::Database::connect(uri.as_str()).await?;

        loom_admin_migrations::Migrator::up(&pool, None).await?;

        Ok(())
    }
}

#[async_trait]
impl Migrate for Provider<ScopeTenant, StateConnected> {
    type Error = Error;

    async fn migrate_database(&self) -> Result<(), <Self as Migrate>::Error> {
        let uri = self.as_ref().connect_options().database_url.clone();
        let pool = sea_orm::Database::connect(uri.as_str()).await?;

        loom_tenant_migrations::Migrator::up(&pool, None).await?;

        Ok(())
    }
}
