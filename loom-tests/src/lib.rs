use embassy_futures::join::join;
use loom_infrastructure::database::{
    Initialize, Migrate,
    database_uri_factory::{self, DatabaseUriType},
};
use loom_infrastructure_impl::{
    Error,
    infrastructure::{
        DatabaseType, Pool, ScopeAdmin, ScopeDefault, ScopeTenant, StateConnected,
    },
};
use tracing::info;
use url::Url;

type ConnectedDefaultPool = Pool<ScopeDefault, StateConnected>;

pub async fn initialize_databases(
    pool: &ConnectedDefaultPool,
    tenant_token: &str,
) -> Result<(), Error> {
    let (admin_result, tenant_result) = join(
        pool.initialize_admin_database(),
        pool.initialize_tenant_database(Some(tenant_token)),
    )
    .await;

    admin_result?;
    tenant_result?;

    Ok(())
}

pub async fn reset_entire_database() -> Result<(), Error> {
    let admin_database_uri =
        database_uri_factory::Factory::new_database_uri(&DatabaseUriType::Admin)
            .get_uri(&DatabaseType::Sqlite.to_string(), None)?
            .to_string()
            .replace("sqlite://", "");
    if std::path::Path::new(&admin_database_uri).exists() {
        std::fs::remove_file(admin_database_uri)?;
    }

    let tenant_template_database_uri =
        database_uri_factory::Factory::new_database_uri(&DatabaseUriType::Tenant)
            .get_uri(&DatabaseType::Sqlite.to_string(), Some("test_token"))?
            .to_string()
            .replace("sqlite://", "");
    if std::path::Path::new(&tenant_template_database_uri).exists() {
        std::fs::remove_file(tenant_template_database_uri)?;
    }

    Ok(())
}

pub async fn get_default_pool() -> Result<Pool<ScopeDefault, StateConnected>, Error> {
    let url = Url::parse("sqlite::memory:").unwrap();
    let default_pool = Pool::connect(&url).await?;
    Ok(default_pool)
}

pub async fn get_admin_pool() -> Result<Pool<ScopeAdmin, StateConnected>, Error> {
    let uri = database_uri_factory::Factory::new_database_uri(&DatabaseUriType::Admin)
        .get_uri(&DatabaseType::Sqlite.to_string(), None)?;
    let admin_pool = Pool::connect(&uri).await?;
    Ok(admin_pool)
}

pub async fn get_tenant_pool(
    tenant_token: &str,
) -> Result<Pool<ScopeTenant, StateConnected>, Error> {
    let uri = database_uri_factory::Factory::new_database_uri(&DatabaseUriType::Tenant)
        .get_uri(&DatabaseType::Sqlite.to_string(), Some(tenant_token))?;
    let tenant_pool = Pool::connect(&uri).await?;
    Ok(tenant_pool)
}

pub async fn refresh_databases(
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

pub mod test_lifecycle {
    pub fn before() {
        dotenvy::from_filename_override(".env.test").expect("Failed to load .env.test.");
    }

    pub fn after() {
        dotenvy::from_filename_override(".env.dev").ok();
    }
}

pub mod test_database_lifecycle {
    use sqlx::any::install_default_drivers;

    use crate::test_lifecycle;

    pub fn before() {
        test_lifecycle::before();

        install_default_drivers();
    }

    pub fn after() {
        test_lifecycle::after();
    }
}
