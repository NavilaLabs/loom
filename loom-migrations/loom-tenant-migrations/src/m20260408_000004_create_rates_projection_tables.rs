use sea_orm_migration::{prelude::*, schema::{pk_uuid, uuid, uuid_null, big_integer, big_integer_null}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Projektsätze: optional nutzerspezifisch (NULL = gilt für alle)
        manager
            .create_table(
                Table::create()
                    .table("projections__project_rates")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(uuid("project_id"))
                    // NULL = Satz gilt für alle User dieses Projekts
                    .col(uuid_null("user_id"))
                    .col(big_integer("hourly_rate"))
                    .col(big_integer_null("internal_rate"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_rates_project_id")
                            .from(
                                TableRef::Table("projections__project_rates".into(), None),
                                "project_id",
                            )
                            .to(TableRef::Table("projections__projects".into(), None), "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table("projections__project_rates")
                    .name("idx_project_rates_project_user")
                    .col("project_id")
                    .col("user_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("projections__activity_rates")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(uuid("activity_id"))
                    .col(uuid_null("user_id"))
                    .col(big_integer("hourly_rate"))
                    .col(big_integer_null("internal_rate"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_activity_rates_activity_id")
                            .from(
                                TableRef::Table("projections__activity_rates".into(), None),
                                "activity_id",
                            )
                            .to(
                                TableRef::Table("projections__activities".into(), None),
                                "id",
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table("projections__activity_rates")
                    .name("idx_activity_rates_activity_user")
                    .col("activity_id")
                    .col("user_id")
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table("projections__activity_rates")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table("projections__project_rates").to_owned())
            .await
    }
}
