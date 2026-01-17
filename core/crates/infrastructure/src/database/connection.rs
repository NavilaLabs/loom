use async_trait::async_trait;

#[async_trait]
pub trait AdminConnection<Connection>
where
    Connection: Send,
{
    type Error;

    async fn acquire_connection(&self) -> Result<Connection, Self::Error>;

    async fn close_connection(&self, connection: Connection);
}

#[async_trait]
pub trait TenantConnection<Connection>
where
    Connection: Send,
{
    type Error;

    async fn acquire_connection(&self) -> Result<Connection, Self::Error>;

    async fn close_connection(&self, connection: Connection);
}
