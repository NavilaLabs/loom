use crate::database::{Error, Initialize, Pool, ScopeDefault, StateConnected};

#[async_trait::async_trait]
impl Initialize for Pool<ScopeDefault, StateConnected> {
    type Error = Error;

    async fn is_initialized(&self) -> Result<bool, <Self as database::Connection>::Error> {
        todo!()
    }

    async fn initialize_admin_database(&self) -> Result<(), <Self as database::Connection>::Error> {
        todo!()
    }

    async fn initialize_tenant_database(
        &self,
    ) -> Result<(), <Self as database::Connection>::Error> {
        todo!()
    }
}
