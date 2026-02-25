use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(loom_admin_migrations::Migrator).await;
}
