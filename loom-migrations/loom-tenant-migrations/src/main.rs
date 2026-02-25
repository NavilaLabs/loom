use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(loom_tenant_migrations::Migrator).await;
}
