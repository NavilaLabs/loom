use infrastructure::Database;

#[async_trait::async_trait]
pub trait Initialize<DB, T>
where
    DB: Send + Sync,
    T: Database<DB> + Send + Sync,
{
    async fn setup_database(&self, database: &T) {
        if let Ok(pool) = database.establish_uninitialized_connection().await {
            self.create_admin_database(&pool).await;
            self.create_tenant_database_template(&pool).await;
            self.run_admin_migrations(&pool).await;
            self.run_tenant_migrations(&pool).await;
        }
    }

    async fn create_admin_database(&self, pool: &DB);

    async fn create_tenant_database_template(&self, pool: &DB);

    async fn run_admin_migrations(&self, pool: &DB);

    async fn run_tenant_migrations(&self, pool: &DB);
}
