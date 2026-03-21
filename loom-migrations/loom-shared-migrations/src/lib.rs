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
        .col(string("event_stream_id").primary_key())
        .col(string("type"))
        .col(integer("version").check(Expr::col("version").gt(0)))
        .col(binary("event"))
        .col(json_binary("metadata"))
        .col(integer("schema_version").check(Expr::col("version").gt(0)))
        .foreign_key(
            ForeignKey::create()
                .name("fk_events_event_stream_id")
                .from(TableRef::Table(name.into(), None), "event_stream_id")
                .to(
                    TableRef::Table("event_streams".into(), None),
                    "event_stream_id",
                ),
        )
        .to_owned();
    let index_create_statements = vec![];

    (table_create_statement, index_create_statements)
}

#[must_use]
pub fn create_event_streams_table_migration() -> (TableCreateStatement, Vec<IndexCreateStatement>) {
    let name = "event_streams";
    let table_create_statement = Table::create()
        .if_not_exists()
        .table(name)
        .col(string("event_stream_id").primary_key())
        .col(integer("version").check(Expr::col("version").gt(0)))
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
        .col(integer("id").primary_key().auto_increment())
        .col(string("event_stream_id"))
        .col(string("aggregate_type"))
        .col(string("aggregate_id"))
        .col(binary("state"))
        .col(integer("version").check(Expr::col("version").gt(0)))
        .col(timestamp("taken_at").default(Expr::current_timestamp()))
        .foreign_key(
            ForeignKey::create()
                .name("fk_snapshots_event_stream_id")
                .from(TableRef::Table(name.into(), None), "event_stream_id")
                .to(
                    TableRef::Table("event_streams".into(), None),
                    "event_stream_id",
                ),
        )
        .to_owned();
    let index_create_statements = vec![
        Index::create()
            .table(name)
            .name("uq_snapshots_aggregate_type_id_version")
            .unique()
            .col("aggregate_type")
            .col("aggregate_id")
            .col("version")
            .to_owned(),
    ];

    (table_create_statement, index_create_statements)
}
