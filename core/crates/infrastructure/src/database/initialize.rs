use async_trait::async_trait;

#[async_trait]
pub trait Initialize<Pool>
where
    Pool: Send,
{
    type Error;

    async fn is_initialized<T>(&self, database: &T) -> bool
    where
        T: super::AdminConnection<Pool> + Send;

    async fn initialize_admin_database(&self, pool: &Pool) -> Result<(), Self::Error>;

    async fn initialize_tenant_database(
        &self,
        pool: &Pool,
        tenant_token: Option<&str>,
    ) -> Result<(), Self::Error>;
}
