use sea_orm_migration::{prelude::*, schema::{pk_uuid, string, string_null, integer, integer_null, big_integer_null, timestamp_with_time_zone, uuid, uuid_null}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("projections__customers")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(string("name"))
                    .col(string_null("comment"))
                    .col(string("currency").string_len(3).default("EUR"))
                    .col(string("timezone").default("Europe/Berlin"))
                    .col(string_null("country").string_len(2))
                    .col(integer("visible").default(1))
                    // Budgets in Sekunden (Zeit) bzw. Cent (Geld), NULL = kein Budget
                    .col(integer_null("time_budget"))
                    .col(big_integer_null("money_budget"))
                    .col(integer("budget_is_monthly").default(0))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("projections__projects")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(uuid("customer_id"))
                    .col(string("name"))
                    .col(string_null("comment"))
                    .col(string_null("order_number"))
                    .col(integer("visible").default(1))
                    .col(integer("billable").default(1))
                    .col(integer_null("time_budget"))
                    .col(big_integer_null("money_budget"))
                    .col(integer("budget_is_monthly").default(0))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_projects_customer_id")
                            .from(
                                TableRef::Table("projections__projects".into(), None),
                                "customer_id",
                            )
                            .to(TableRef::Table("projections__customers".into(), None), "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table("projections__projects")
                    .name("idx_projects_customer_id")
                    .col("customer_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("projections__activities")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    // NULL = globale Aktivität (projektübergreifend)
                    .col(uuid_null("project_id"))
                    .col(string("name"))
                    .col(string_null("comment"))
                    .col(integer("visible").default(1))
                    .col(integer("billable").default(1))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_activities_project_id")
                            .from(
                                TableRef::Table("projections__activities".into(), None),
                                "project_id",
                            )
                            .to(TableRef::Table("projections__projects".into(), None), "id")
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table("projections__activities")
                    .name("idx_activities_project_id")
                    .col("project_id")
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("projections__activities").to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table("projections__projects").to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table("projections__customers").to_owned())
            .await
    }
}
