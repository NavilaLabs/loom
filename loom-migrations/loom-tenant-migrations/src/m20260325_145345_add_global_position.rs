use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        #[cfg(feature = "postgres")]
        db.execute_unprepared(
            "CREATE SEQUENCE IF NOT EXISTS events_global_position_seq;

            ALTER TABLE events
                ADD COLUMN global_position BIGINT NOT NULL DEFAULT nextval('events_global_position_seq');

            CREATE INDEX IF NOT EXISTS events_global_position_idx
                ON events (global_position);",
        )
        .await?;
        #[cfg(feature = "sqlite")]
        db.execute_unprepared(
            "-- Add global_position column if it doesn't exist
            ALTER TABLE events ADD COLUMN global_position INTEGER NOT NULL DEFAULT 0;

            -- Back-fill existing rows using rowid as a stable insertion-order integer
            UPDATE events SET global_position = rowid;

            -- Recreate trigger to assign global_position on insert
            DROP TRIGGER IF EXISTS events_assign_global_position;

            CREATE TRIGGER events_assign_global_position
            AFTER INSERT ON events
            FOR EACH ROW
            WHEN NEW.global_position = 0
            BEGIN
            UPDATE events
            SET global_position = (
                SELECT COALESCE(MAX(global_position), 0) + 1
                FROM events
                WHERE global_position != 0
            )
            WHERE rowid = NEW.rowid;
            END;

            -- Index for efficient querying by global_position
            CREATE INDEX IF NOT EXISTS events_global_position_idx
            ON events (global_position);",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        #[cfg(feature = "postgres")]
        db.execute_unprepared(
            "DROP INDEX IF EXISTS events_global_position_idx;

            ALTER TABLE events DROP COLUMN IF EXISTS global_position;

            DROP SEQUENCE IF EXISTS events_global_position_seq;",
        )
        .await?;
        #[cfg(feature = "sqlite")]
        db.execute_unprepared(
            "DROP TRIGGER IF EXISTS events_assign_global_position;

            DROP INDEX IF EXISTS events_global_position_idx;

            -- SQLite does not support DROP COLUMN before version 3.35.0.
            -- If your SQLite version is >= 3.35.0, uncomment the line below instead.
            -- ALTER TABLE events DROP COLUMN global_position;

            -- Fallback: recreate the table without the global_position column
            CREATE TABLE events_backup AS
            SELECT * FROM events;

            DROP TABLE events;

            ALTER TABLE events_backup RENAME TO events;",
        )
        .await?;

        Ok(())
    }
}
