use async_trait::async_trait;
use infrastructure::event_store::GetLastAggregateRecord;
use modules::shared::value_objects::{AggregateId, AggregateType};
use sea_orm::ExprTrait;
use sea_query::{Alias, Expr, PostgresQueryBuilder, Query, SqliteQueryBuilder, TableName};
use sea_query_sqlx::SqlxBinder;

use crate::sea_query_sqlx::infrastructure::{DatabaseType, EventRecord, Provider, StateConnected};

#[async_trait]
impl<Scope> GetLastAggregateRecord for Provider<Scope, StateConnected>
where
    Scope: Send + Sync,
{
    async fn get_last_aggregate_record(
        &self,
        aggregate_type: &AggregateType,
        aggregate_id: &AggregateId,
    ) -> Result<Option<infrastructure::EventRecord>, crate::Error> {
        let query = Query::select()
            .from(TableName::from("events"))
            .and_where(Expr::col("aggregate_type").eq(aggregate_type.to_string()))
            .and_where(
                Expr::col("aggregate_id")
                    .eq(Expr::val(aggregate_id.to_string()).cast_as(Alias::new("uuid"))),
            )
            .to_owned();

        dbg!(&query.build_sqlx(PostgresQueryBuilder));

        let (sql, values) = match self.get_database_type() {
            DatabaseType::Postgres => query.build_sqlx(PostgresQueryBuilder),
            DatabaseType::Sqlite => query.build_sqlx(SqliteQueryBuilder),
        };

        let record = sqlx::query_with(&sql, values)
            .fetch_optional(self.as_ref())
            .await?;

        match record {
            Some(record) => Ok(Some(EventRecord::try_from(record)?.0)),
            None => Ok(None),
        }
    }
}
