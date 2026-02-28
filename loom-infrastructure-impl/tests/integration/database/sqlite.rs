use loom_infrastructure::database::{
    Migrate,
    database_uri_factory::{self, DatabaseScope},
};
use loom_infrastructure_impl::{
    Error,
    infrastructure::{Pool, ScopeAdmin, ScopeDefault, ScopeTenant, StateConnected},
};
use tracing::info;
use url::Url;

use super::ConnectedDefaultPool;
use super::initialize_databases;

async fn reset_entire_database() -> Result<(), Error> {
    let admin_database_uri = database_uri_factory::Factory::new_database_uri(&DatabaseScope::Admin)
        .get_uri("sqlite", None)?
        .to_string()
        .replace("sqlite://", "");
    if std::path::Path::new(&admin_database_uri).exists() {
        std::fs::remove_file(admin_database_uri)?;
    }

    let tenant_template_database_uri =
        database_uri_factory::Factory::new_database_uri(&DatabaseScope::Tenant)
            .get_uri("sqlite", Some("test_token"))?
            .to_string()
            .replace("sqlite://", "");
    if std::path::Path::new(&tenant_template_database_uri).exists() {
        std::fs::remove_file(tenant_template_database_uri)?;
    }

    Ok(())
}

async fn get_default_pool() -> Result<Pool<ScopeDefault, StateConnected>, Error> {
    let url = Url::parse("sqlite::memory:").unwrap();
    let default_pool = Pool::connect(&url).await?;
    Ok(default_pool)
}

async fn get_admin_pool() -> Result<Pool<ScopeAdmin, StateConnected>, Error> {
    let uri = database_uri_factory::Factory::new_database_uri(&DatabaseScope::Admin)
        .get_uri("sqlite", None)?;
    dbg!(&uri);
    let admin_pool = Pool::connect(&uri).await?;
    Ok(admin_pool)
}

async fn get_tenant_pool(
    tenant_token: &str,
) -> Result<Pool<ScopeTenant, StateConnected>, Error> {
    let uri = database_uri_factory::Factory::new_database_uri(&DatabaseScope::Tenant)
        .get_uri("sqlite", Some(tenant_token))?;
    let tenant_pool = Pool::connect(&uri).await?;
    Ok(tenant_pool)
}

pub(crate) async fn refresh_databases(
    pool: &ConnectedDefaultPool,
    tenant_token: &str,
) -> Result<(), Error> {
    reset_entire_database().await?;
    info!("Database successfully reseted");
    initialize_databases(pool, tenant_token).await?;
    info!("Database successfully initialized");

    let admin_pool = get_admin_pool().await?;
    admin_pool.migrate_database().await?;
    let tenant_pool = get_tenant_pool(tenant_token).await?;
    tenant_pool.migrate_database().await?;
    info!("Database successfully migrated");

    Ok(())
}

pub mod tests {
    use with_lifecycle::with_lifecycle;

    use crate::database::test_lifecycle;

    use super::*;

    #[with_lifecycle(test_lifecycle)]
    #[tokio::test]
    async fn test_setup_sqlite_database() -> Result<(), Error> {
        let default_pool = get_default_pool().await?;
        refresh_databases(&default_pool, "test_token").await?;

        Ok(())
    }
}
