use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("projections__timesheets")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(uuid("user_id"))
                    .col(uuid("project_id"))
                    .col(uuid("activity_id"))
                    .col(timestamp_with_time_zone("start_time"))
                    // NULL solange die Buchung läuft
                    .col(timestamp_with_time_zone_null("end_time"))
                    // Dauer in Sekunden; NULL bei laufender Buchung
                    .col(integer_null("duration"))
                    .col(string_null("description"))
                    .col(string("timezone").default("Europe/Berlin"))
                    .col(integer("billable").default(1))
                    // Snapshot der Stundensätze zum Buchungszeitpunkt in Cent
                    .col(big_integer_null("hourly_rate"))
                    .col(big_integer_null("fixed_rate"))
                    .col(big_integer_null("internal_rate"))
                    // Gesamtbetrag = hourly_rate * duration / 3600, in Cent
                    .col(big_integer_null("rate"))
                    // true sobald in einer Rechnung exportiert — sperrt den Eintrag
                    .col(integer("exported").default(0))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()))
                    // No FK on user_id — users live in the admin database (cross-DB FK not supported).
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

        // Häufigste Abfragen: alle Buchungen eines Users, gefiltert nach Zeitraum
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

        // Laufende Buchungen (end_time IS NULL) schnell findbar
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
        manager
            .drop_table(Table::drop().table("projections__timesheets").to_owned())
            .await
    }
}
