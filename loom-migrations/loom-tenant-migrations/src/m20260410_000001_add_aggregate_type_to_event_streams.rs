use sea_orm_migration::prelude::*;

/// Adds `aggregate_type` to `event_streams` and a unique index on
/// `(aggregate_type, event_stream_id)`.  See the admin migration of the same
/// name for the full rationale.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        conn.execute_unprepared(
            "ALTER TABLE event_streams ADD COLUMN aggregate_type TEXT NOT NULL DEFAULT ''",
        )
        .await?;

        manager
            .create_index(
                Index::create()
                    .table("event_streams")
                    .name("uq_event_streams_aggregate_type_stream_id")
                    .unique()
                    .col("aggregate_type")
                    .col("event_stream_id")
                    .if_not_exists()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("uq_event_streams_aggregate_type_stream_id")
                    .table("event_streams")
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE event_streams DROP COLUMN aggregate_type")
            .await?;

        Ok(())
    }
}
