use async_trait::async_trait;

#[async_trait]
pub trait Handler<Pool, Command>
where
    Pool: Send,
{
    type Error;

    async fn handle(&self, pool: &Pool, command: Command) -> Result<(), Self::Error>;
}
