use std::fmt::Display;

use crate::config::CONFIG;

#[async_trait::async_trait]
pub trait Database<T>
where
    T: Send + Sync,
{
    type Error: Display + Send + Sync;

    async fn establish_admin_connection(&self) -> Result<T, Self::Error> {
        self.establish_connection(CONFIG.get_database().get_databases().get_admin().get_name())
            .await
    }

    async fn establish_tenant_connection(&self, tenant_token: &str) -> Result<T, Self::Error> {
        let tenant_db = format!(
            "{}{}",
            CONFIG.get_database().get_databases().get_admin().get_name(),
            tenant_token
        );

        self.establish_connection(&tenant_db).await
    }

    async fn establish_uninitialized_connection(&self) -> Result<T, Self::Error> {
        self.establish_connection("postgres").await
    }

    async fn establish_connection(&self, database: &str) -> Result<T, Self::Error>;
}
