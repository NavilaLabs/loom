use async_trait::async_trait;
use domain::tenant::value_objects::TenantToken;

#[async_trait]
pub trait Initialize<Connection>
where
    Connection: Send,
{
    type Error;

    async fn is_initialized(&self, connection: &Connection) -> Result<bool, Self::Error>;

    async fn initialize_admin_database(
        &self,
        connection: &mut Connection,
    ) -> Result<(), Self::Error>;

    async fn initialize_tenant_database(
        &self,
        connection: &mut Connection,
        tenant_token: Option<&TenantToken>,
    ) -> Result<(), Self::Error>;
}
