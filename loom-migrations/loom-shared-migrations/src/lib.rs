use sea_orm_migration::{prelude::*, schema::*};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error: {0}")]
    DbError(#[from] DbErr),
}

#[must_use]
pub fn create_events_table_migration() -> (TableCreateStatement, Vec<IndexCreateStatement>) {
    let name = "events";
    let table_create_statement = Table::create()
        .if_not_exists()
        .table(name)
        .col(
            big_integer("global_position")
                .primary_key()
                .auto_increment()
                .not_null(),
        )
        .col(uuid("event_stream_id"))
        // event columns
        .col(uuid("event_id"))
        .col(string("event_type"))
        .col(integer("event_schema_version"))
        // aggregate columns
        .col(string("aggregate_type"))
        .col(uuid("aggregate_id"))
        .col(integer("aggregate_version"))
        .col(integer("aggregate_schema_version"))
        // data columns
        .col(json_binary("data"))
        .col(json_binary_null("metadata"))
        // timestamp columns
        .col(timestamp("created_at").default(Expr::current_timestamp()))
        .col(timestamp_null("effective_at"))
        // tracing columns
        .col(uuid_null("correlation_id"))
        .col(uuid_null("causation_id"))
        .foreign_key(
            ForeignKey::create()
                .name("fk_events_event_stream_id")
                .from(TableRef::Table(name.into(), None), "event_stream_id")
                .to(TableRef::Table("event_streams".into(), None), "id"),
        )
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
        Index::create()
            .table(name)
            .name("idx_events_correlation_id")
            .col("correlation_id")
            .to_owned(),
        Index::create()
            .table(name)
            .name("idx_events_causation_id")
            .col("causation_id")
            .to_owned(),
    ];

    (table_create_statement, index_create_statements)
}

#[must_use]
pub fn create_event_streams_table_migration() -> (TableCreateStatement, Vec<IndexCreateStatement>) {
    let name = "event_streams";
    let table_create_statement = Table::create()
        .if_not_exists()
        .table(name)
        .col(uuid("id").primary_key())
        .col(integer("version"))
        .to_owned();
    let index_create_statements = vec![];

    (table_create_statement, index_create_statements)
}

#[must_use]
pub fn create_snapshots_table_migration() -> (TableCreateStatement, Vec<IndexCreateStatement>) {
    let name = "snapshots";
    let table_create_statement = Table::create()
        .if_not_exists()
        .table(name)
        .col(uuid("event_stream_id"))
        // snapshot columns
        .col(uuid("id").primary_key())
        .col(string("aggregate_type"))
        .col(uuid("aggregate_id"))
        .col(integer("aggregate_version"))
        .col(json_binary("data"))
        .col(json_binary_null("metadata"))
        .col(timestamp("created_at").default(Expr::current_timestamp()))
        .foreign_key(
            ForeignKey::create()
                .name("fk_snapshots_event_stream_id")
                .from(TableRef::Table(name.into(), None), "event_stream_id")
                .to(TableRef::Table("event_streams".into(), None), "id"),
        )
        .to_owned();
    let index_create_statements = vec![
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
            .name("idx_snapshots_lookup")
            .col("aggregate_type")
            .col("aggregate_id")
            .to_owned(),
        Index::create()
            .table(name)
            .name("idx_snapshots_created_at")
            .col("created_at")
            .to_owned(),
    ];

    (table_create_statement, index_create_statements)
}
