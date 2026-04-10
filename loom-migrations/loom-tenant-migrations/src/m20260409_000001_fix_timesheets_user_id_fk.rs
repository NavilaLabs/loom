use sea_orm_migration::{prelude::*, schema::{pk_uuid, uuid, timestamp_with_time_zone, timestamp_with_time_zone_null, integer_null, string_null, string, integer, big_integer_null}};

#[derive(DeriveMigrationName)]
pub struct Migration;

/// `SQLite` does not support `ALTER TABLE DROP CONSTRAINT`, so we have to recreate
/// the table without the erroneous FK on `user_id` (users live in the admin DB,
/// not in the tenant DB, so the FK can never be satisfied).
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 1. Rename the existing table so we can repopulate it.
        db.execute_unprepared(
            r#"ALTER TABLE "projections__timesheets" RENAME TO "projections__timesheets_old""#,
        )
        .await?;

        // 2. Recreate the table without the user_id FK.
        manager
            .create_table(
                Table::create()
                    .table("projections__timesheets")
                    .col(pk_uuid("id"))
                    .col(uuid("user_id"))
                    .col(uuid("project_id"))
                    .col(uuid("activity_id"))
                    .col(timestamp_with_time_zone("start_time"))
                    .col(timestamp_with_time_zone_null("end_time"))
                    .col(integer_null("duration"))
                    .col(string_null("description"))
                    .col(string("timezone").default("Europe/Berlin"))
                    .col(integer("billable").default(1))
                    .col(big_integer_null("hourly_rate"))
                    .col(big_integer_null("fixed_rate"))
                    .col(big_integer_null("internal_rate"))
                    .col(big_integer_null("rate"))
                    .col(integer("exported").default(0))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()))
                    // No FK on user_id — users live in the admin database.
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_timesheets_project_id")
                            .from(
                                TableRef::Table("projections__timesheets".into(), None),
                                "project_id",
                            )
                            .to(TableRef::Table("projections__projects".into(), None), "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_timesheets_activity_id")
                            .from(
                                TableRef::Table("projections__timesheets".into(), None),
                                "activity_id",
                            )
                            .to(
                                TableRef::Table("projections__activities".into(), None),
                                "id",
                            )
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        // 3. Copy all existing rows.
        db.execute_unprepared(
            r#"INSERT INTO "projections__timesheets"
               SELECT "id", "user_id", "project_id", "activity_id",
                      "start_time", "end_time", "duration", "description",
                      "timezone", "billable", "hourly_rate", "fixed_rate",
                      "internal_rate", "rate", "exported", "created_at", "updated_at"
               FROM "projections__timesheets_old""#,
        )
        .await?;

        // 4. Drop the old table.
        db.execute_unprepared(r#"DROP TABLE "projections__timesheets_old""#)
            .await?;

        // 5. Recreate indexes.
        manager
            .create_index(
                Index::create()
                    .table("projections__timesheets")
                    .name("idx_timesheets_user_start")
                    .col("user_id")
                    .col("start_time")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table("projections__timesheets")
                    .name("idx_timesheets_project_id")
                    .col("project_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table("projections__timesheets")
                    .name("idx_timesheets_exported")
                    .col("exported")
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Reverting would require re-adding the bad FK, which is intentionally not done.
        // Simply a no-op; data is preserved in the table created by `up`.
        let _ = manager;
        Ok(())
    }
}
