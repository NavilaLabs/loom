use domain::tenant::value_objects::TenantToken;
use infrastructure::database::Migrate;
use infrastructure::database::{
    Error, Pool, ScopeAdmin, ScopeDefault, ScopeTenant, StateConnected,
};
use shared::build_tenant_database_name;
use sqlx::ConnectOptions;
use sqlx::sqlite::SqliteConnectOptions;
use url::Url;

use super::ConnectedDefaultPool;
use super::initialize_databases;

const ADMIN_DATABASE_PATH: &str = "/workspaces/loom/test_loom_admin.sqlite";
const TENANT_TEMPLATE_DATABASE_PATH: &str = "/workspaces/loom/test_loom_tenant_template.sqlite";

async fn reset_entire_database() -> Result<(), Error> {
    if std::path::Path::new(ADMIN_DATABASE_PATH).exists() {
        std::fs::remove_file(ADMIN_DATABASE_PATH)?;
    }
    if std::path::Path::new(TENANT_TEMPLATE_DATABASE_PATH).exists() {
        std::fs::remove_file(TENANT_TEMPLATE_DATABASE_PATH)?;
    }

    Ok(())
}

async fn get_default_pool() -> Result<Pool<ScopeDefault, StateConnected>, Error> {
    let url = Url::parse("sqlite::memory:").unwrap();
    let default_pool = Pool::connect(&url).await?;
    Ok(default_pool)
}

async fn get_admin_pool() -> Result<Pool<ScopeAdmin, StateConnected>, Error> {
    let url = SqliteConnectOptions::new()
        .filename("/workspaces/loom/test_loom_admin.sqlite")
        .to_url_lossy();
    let admin_pool = Pool::connect(&url).await?;
    Ok(admin_pool)
}

async fn get_tenant_pool(
    tenant_token: Option<&TenantToken>,
) -> Result<Pool<ScopeTenant, StateConnected>, Error> {
    let db_name = build_tenant_database_name("loom_tenant", tenant_token);
    let url = SqliteConnectOptions::new()
        .filename(format!("/workspaces/loom/test_{}.sqlite", db_name))
        .to_url_lossy();
    let tenant_pool = Pool::connect(&url).await?;
    Ok(tenant_pool)
}

pub(crate) async fn refresh_databases(pool: &ConnectedDefaultPool) -> Result<(), Error> {
    reset_entire_database().await?;
    initialize_databases(pool).await?;

    let admin_pool = get_admin_pool().await?;
    admin_pool.migrate_database().await?;
    let tenant_template_pool = get_tenant_pool(None).await?;
    tenant_template_pool.migrate_database().await?;

    Ok(())
}

pub mod tests {
    use with_lifecycle::with_lifecycle;

    use crate::database::test_lifecycle;

    use super::*;

    #[with_lifecycle(test_lifecycle)]
    #[tokio::test]
    async fn test_setup_postgres_database() -> Result<(), Error> {
        let default_pool = get_default_pool().await?;
        refresh_databases(&default_pool).await?;

        Ok(())
    }
}
