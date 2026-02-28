use sea_orm_migration::{prelude::*, schema::*};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error: {0}")]
    DbError(#[from] DbErr),
}

#[must_use]
pub fn create_events_table_migration(
    name: Option<&'static str>,
) -> (TableCreateStatement, Vec<IndexCreateStatement>) {
    let name = name.unwrap_or("events");
    let table_create_statement = Table::create()
        .if_not_exists()
        .table(name)
        .col(
            big_integer("global_position")
                .primary_key()
                .auto_increment()
                .not_null(),
        )
        // event columns
        .col(uuid("event_id"))
        .col(string("event_type"))
        // aggregate columns
        .col(string("aggregate_type"))
        .col(uuid("aggregate_id"))
        .col(integer("aggregate_version"))
        // data columns
        .col(json_binary("data"))
        .col(json_binary_null("metadata"))
        // timestamp columns
        .col(timestamp("created_at").default(Expr::current_timestamp()))
        .to_owned();
    let index_create_statements = vec![
        Index::create()
            .table(name)
            .name("uq_events_event_id")
            .unique()
            .col("event_id")
            .to_owned(),
        Index::create()
            .table(name)
            .name("uq_events_aggregate_type_id_version")
            .unique()
            .col("aggregate_type")
            .col("aggregate_id")
            .col("aggregate_version")
            .to_owned(),
    ];

    (table_create_statement, index_create_statements)
}

#[must_use]
pub fn create_snapshots_table_migration(
    name: Option<&'static str>,
) -> (TableCreateStatement, Vec<IndexCreateStatement>) {
    let name = name.unwrap_or("snapshots");
    let table_create_statement = Table::create()
        .if_not_exists()
        .table(name)
        // snapshot columns
        .col(string("aggregate_type"))
        .col(uuid("aggregate_id"))
        .col(integer("aggregate_version"))
        .col(json_binary("state"))
        .col(timestamp("created_at").default(Expr::current_timestamp()))
        .col(timestamp("updated_at").default(Expr::current_timestamp()))
        .to_owned();
    let index_create_statements = vec![
        Index::create()
            .table(name)
            .name("pk_snapshots_aggregate_type_id")
            .unique()
            .col("aggregate_type")
            .col("aggregate_id")
            .to_owned(),
        Index::create()
            .table(name)
            .name("uq_snapshots_aggregate_type_id_version")
            .unique()
            .col("aggregate_type")
            .col("aggregate_id")
            .col("aggregate_version")
            .to_owned(),
        Index::create()
            .table(name)
            .name("idx_snapshots_created_at")
            .col("created_at")
            .to_owned(),
        Index::create()
            .table(name)
            .name("idx_snapshots_updated_at")
            .col("updated_at")
            .to_owned(),
        Index::create()
            .table(name)
            .name("idx_snapshots_created_at_updated_at")
            .col("created_at")
            .col("updated_at")
            .to_owned(),
    ];

    (table_create_statement, index_create_statements)
}
