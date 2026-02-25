pub use loom_shared_migrations::Error;
pub use sea_orm_migration::prelude::*;

mod m20251214_102155_create_events_table;
mod m20251215_183826_create_snapshots_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251214_102155_create_events_table::Migration),
            Box::new(m20251215_183826_create_snapshots_table::Migration),
        ]
    }
}
