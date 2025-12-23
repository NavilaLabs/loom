use sea_orm_migration::{prelude::*, schema::*};

#[must_use]
pub fn create_events_table_migration(name: Option<&'static str>) -> TableCreateStatement {
    Table::create()
        .if_not_exists()
        .table(name.unwrap_or("events"))
        // event columns
        .col(uuid("event_id").primary_key())
        .col(string("event_type"))
        .col(integer("event_version"))
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
        // user columns
        .col(uuid("created_by"))
        .col(uuid_null("owned_by"))
        // tracing columns
        .col(uuid_null("correlation_id"))
        .col(uuid_null("causation_id"))
        // integrity columns
        .col(binary("hash"))
        // indexes
        .index(
            Index::create()
                .name("uq_events_aggregate_type_id_version")
                .unique()
                .col("aggregate_type")
                .col("aggregate_id")
                .col("aggregate_version"),
        )
        .index(
            Index::create()
                .name("idx_events_correlation_id")
                .col("correlation_id"),
        )
        .index(
            Index::create()
                .name("idx_events_causation_id")
                .col("causation_id"),
        )
        .index(
            Index::create()
                .name("idx_events_event_type")
                .col("event_type"),
        )
        .index(
            Index::create()
                .name("idx_events_event_type_version")
                .col("event_type")
                .col("event_version"),
        )
        .to_owned()
}

#[must_use]
pub fn create_snapshots_table_migration(name: Option<&'static str>) -> TableCreateStatement {
    Table::create()
        .if_not_exists()
        .table(name.unwrap_or("snapshots"))
        // snapshot columns
        .col(uuid("snapshot_id").primary_key())
        .col(string("aggregate_type"))
        .col(uuid("aggregate_id"))
        .col(integer("aggregate_version"))
        .col(integer("aggregate_schema_version"))
        .col(json_binary("data"))
        .col(json_binary("metadata"))
        .col(timestamp("created_at").default(Expr::current_timestamp()))
        // indexes
        .index(
            Index::create()
                .name("uq_snapshots_aggregate_type_id_version")
                .unique()
                .col("aggregate_type")
                .col("aggregate_id")
                .col("aggregate_version"),
        )
        .index(
            Index::create()
                .name("idx_snapshots_lookup")
                .col("aggregate_type")
                .col("aggregate_id"),
        )
        .to_owned()
}
