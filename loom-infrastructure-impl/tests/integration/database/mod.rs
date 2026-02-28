use embassy_futures::join::join;
use loom_infrastructure::database::Initialize;
use loom_infrastructure_impl::{
    Error,
    infrastructure::{Pool, ScopeDefault, StateConnected},
};

pub mod postgres;
pub mod sqlite;

type ConnectedDefaultPool = Pool<ScopeDefault, StateConnected>;

async fn initialize_databases(
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

pub(crate) mod test_lifecycle {
    use loom_shared::test_lifecycle;
    use sqlx::any::install_default_drivers;

    pub fn before() {
        test_lifecycle::before();

        install_default_drivers();
    }

    pub fn after() {
        test_lifecycle::after();
    }
}
