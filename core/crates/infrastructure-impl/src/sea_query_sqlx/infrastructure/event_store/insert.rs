use async_trait::async_trait;
use infrastructure::{EventRecord, event_store::Insert};
use sea_query::{PostgresQueryBuilder, Query, SqliteQueryBuilder};
use sea_query_sqlx::SqlxBinder;

use crate::sea_query_sqlx::infrastructure::{DatabaseType, Provider, StateConnected};

#[async_trait]
impl<Scope> Insert for Provider<Scope, StateConnected>
where
    Scope: Send + Sync,
{
    async fn insert(&self, record: EventRecord) -> Result<(), crate::Error> {
        let query = Query::insert()
            .into_table("events")
            .columns([
                "event_id",
                "event_type",
                "event_version",
                "aggregate_type",
                "aggregate_id",
                "aggregate_version",
                "data",
                "metadata",
                "created_at",
                "effective_at",
                "created_by",
                "owned_by",
                "correlation_id",
                "causation_id",
                "hash",
                "previous_hash",
            ])
            .values_panic([
                self.cast_uuid(record.get_event_id().to_string()).into(),
                record.get_event_type().into(),
                record.get_event_version().into(),
                record.get_aggregate().get_type().into(),
                self.cast_uuid(record.get_aggregate().get_id().to_string())
                    .into(),
                record.get_aggregate().get_version().into(),
                self.cast_jsonb(serde_json::to_string(&record.get_data())?)
                    .into(),
                self.cast_jsonb(serde_json::to_string(&record.get_metadata())?)
                    .into(),
                self.cast_timestamp(record.get_timestamps().get_created_at().to_string())
                    .into(),
                self.cast_timestamp_opt(
                    record
                        .get_timestamps()
                        .get_effective_at()
                        .map(|t| t.to_string()),
                )
                .into(),
                self.cast_uuid(record.get_context().get_created_by().to_string())
                    .into(),
                self.cast_uuid_opt(record.get_context().get_owned_by().map(|id| id.to_string()))
                    .into(),
                self.cast_uuid_opt(
                    record
                        .get_context()
                        .get_correlation_id()
                        .map(|id| id.to_string()),
                )
                .into(),
                self.cast_uuid_opt(
                    record
                        .get_context()
                        .get_causation_id()
                        .map(|id| id.to_string()),
                )
                .into(),
                record.get_hash().into(),
                record.get_previous_hash().into(),
            ])
            .to_owned();

        let (sql, values) = match self.get_database_type() {
            DatabaseType::Postgres => query.build_sqlx(PostgresQueryBuilder),
            DatabaseType::Sqlite => query.build_sqlx(SqliteQueryBuilder),
        };

        sqlx::query_with(&sql, values)
            .execute(self.as_ref())
            .await?;

        Ok(())
    }
}
