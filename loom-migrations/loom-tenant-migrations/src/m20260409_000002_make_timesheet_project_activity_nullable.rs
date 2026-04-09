use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

/// Makes `project_id` and `activity_id` nullable and removes their FK constraints
/// so that timesheets can be started without a project/activity ("quick timer").
/// Uses the rename-recreate-copy pattern required by SQLite.
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"ALTER TABLE "projections__timesheets" RENAME TO "projections__timesheets_old""#,
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table("projections__timesheets")
                    .col(pk_uuid("id"))
                    .col(uuid("user_id"))
                    // nullable — quick timer starts without a project
                    .col(uuid_null("project_id"))
                    // nullable — quick timer starts without an activity
                    .col(uuid_null("activity_id"))
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
                    .col(
                        timestamp_with_time_zone("created_at")
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone("updated_at")
                            .default(Expr::current_timestamp()),
                    )
                    // No FK constraints — project/activity are optional
                    .to_owned(),
            )
            .await?;

        db.execute_unprepared(
            r#"INSERT INTO "projections__timesheets"
               SELECT "id", "user_id", "project_id", "activity_id",
                      "start_time", "end_time", "duration", "description",
                      "timezone", "billable", "hourly_rate", "fixed_rate",
                      "internal_rate", "rate", "exported", "created_at", "updated_at"
               FROM "projections__timesheets_old""#,
        )
        .await?;

        db.execute_unprepared(r#"DROP TABLE "projections__timesheets_old""#)
            .await?;

        manager
            .create_index(
                Index::create()
                    .table("projections__timesheets")
                    .name("idx_timesheets_user_start_v2")
                    .col("user_id")
                    .col("start_time")
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
