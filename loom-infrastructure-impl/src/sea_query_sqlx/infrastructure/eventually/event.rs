use async_trait::async_trait;
use eventually::{
    event,
    message::{Envelope, Message},
    serde,
    version::{self, Version},
};
use futures_util::{StreamExt, TryStreamExt, future::ready};
use loom_infrastructure::event_sourcing::Metadata;
use sea_orm::ExprTrait;
use sea_query::{Expr, PostgresQueryBuilder, Query, SqliteQueryBuilder};
use sea_query_sqlx::SqlxBinder;
use sqlx::{Any, Row, Transaction, any::AnyRow};
use uuid::Uuid;

use crate::infrastructure::{DatabaseType, StateConnected};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to get column '{name}' from result row: {error}")]
    ReadColumn {
        name: &'static str,
        #[source]
        error: sqlx::Error,
    },
    #[error("{0}")]
    SerializeEventError(String),
}

pub(crate) async fn append_domain_event<Evt>(
    tx: &mut Transaction<'_, Any>,
    serde: &impl serde::Serializer<Evt>,
    event_stream_id: &str,
    event_version: i32,
    new_event_stream_version: i32,
    event: event::Envelope<Evt>,
) -> Result<(), crate::infrastructure::Error>
where
    Evt: Message,
{
    let event_type = event.message.name();
    let metadata = event.metadata;
    let serialized_event = serde
        .serialize(event.message)
        .map_err(|err| Error::SerializeEventError(err.to_string()))?;

    todo!()
}

pub struct EventStore<Scope, Id, Evt, Serde> {
    pool: crate::infrastructure::Pool<Scope, StateConnected>,
    aggregate_type: String,
    serde: Serde,
    _id: std::marker::PhantomData<Id>,
    _evt: std::marker::PhantomData<Evt>,
}

impl<Scope, Id, Evt, Serde> EventStore<Scope, Id, Evt, Serde>
where
    Scope: Send + Sync,
    Id: ToString + Clone + Send + Sync,
    Evt: Message + Send + Sync,
    Serde: serde::Serde<Evt> + Send + Sync,
{
    fn event_row_to_persisted_event(
        &self,
        stream_id: Id,
        row: &AnyRow,
    ) -> Result<event::Persisted<Id, Evt>, crate::infrastructure::Error> {
        let version_column: i32 = row.try_get("aggregate_version")?;
        let data_column: Vec<u8> = row.try_get("data")?;
        let metadata_column: Vec<u8> = row.try_get("metadata")?;

        let data: Evt = self
            .serde
            .deserialize(&data_column)
            .map_err(|err| crate::infrastructure::Error::DeserializeEventError(err.to_string()))?;
        let metadata: Metadata = serde_json::from_slice(&metadata_column)?;
        let metadata: eventually::message::Metadata =
            serde_json::from_value(serde_json::to_value(metadata)?)?;

        Ok(event::Persisted {
            stream_id,
            version: version_column as Version,
            event: Envelope {
                message: data,
                metadata,
            },
        })
    }
}

impl<Scope, Id, Evt, Serde> event::store::Streamer<Id, Evt> for EventStore<Scope, Id, Evt, Serde>
where
    Scope: Send + Sync,
    Id: ToString + Clone + Send + Sync,
    Evt: Message + Send + Sync,
    Serde: serde::Serde<Evt> + Send + Sync,
{
    type Error = crate::infrastructure::Error;

    fn stream(&self, id: &Id, select: event::VersionSelect) -> event::Stream<Id, Evt, Self::Error> {
        let aggregate_id = id.to_string();
        let aggregate_id_uuid = Uuid::parse_str(&aggregate_id).unwrap();

        let from_version = match select {
            event::VersionSelect::All => 0,
            event::VersionSelect::From(v) => v as i32,
        };

        let sql = match self.pool.get_database_type() {
            DatabaseType::Postgres => {
                "SELECT aggregate_version, data, metadata FROM events WHERE aggregate_type = $1 AND aggregate_id = $2 AND aggregate_version >= $3 ORDER BY aggregate_version ASC"
            }
            DatabaseType::Sqlite => {
                "SELECT aggregate_version, data, metadata FROM events WHERE aggregate_type = ? AND aggregate_id = ? AND aggregate_version >= ? ORDER BY aggregate_version ASC"
            }
        };

        let id = id.clone();

        sqlx::query(sql)
            .bind(self.aggregate_type.clone())
            .bind(aggregate_id_uuid.to_string())
            .bind(from_version)
            .fetch(self.pool.as_ref())
            .map_err(crate::infrastructure::Error::from)
            .and_then(move |row| ready(self.event_row_to_persisted_event(id.clone(), &row)))
            .boxed()
    }
}

#[async_trait::async_trait]
impl<Scope, Id, Evt, Serde> event::store::Appender<Id, Evt> for EventStore<Scope, Id, Evt, Serde>
where
    Scope: Send + Sync,
    Id: ToString + Clone + Send + Sync,
    Evt: Message + Send + Sync,
    Serde: serde::Serde<Evt> + Send + Sync,
{
    async fn append(
        &self,
        id: Id,
        version_check: version::Check,
        events: Vec<event::Envelope<Evt>>,
    ) -> Result<Version, event::store::AppendError> {
        if events.is_empty() {
            return Ok(0); // Optional: Aktuelle Version abfragen
        }

        let mut tx = self
            .pool
            .as_ref()
            .begin()
            .await
            .map_err(|e| event::store::AppendError::Internal(e.into()))?;
        let aggregate_id = id.to_string();
        let aggregate_id_uuid = Uuid::parse_str(&aggregate_id).unwrap();

        // 1. Finde die Start-Version heraus
        let current_version = match version_check {
            version::Check::MustBe(v) => v as i32,
            version::Check::Any => {
                let mut query = Query::select();
                query
                    .expr(Expr::col(Events::AggregateVersion).max())
                    .from(Events::Table)
                    .and_where(Expr::col(Events::AggregateType).eq(self.aggregate_type.clone()))
                    .and_where(Expr::col(Events::AggregateId).eq(aggregate_id_uuid));

                let (sql, values) = match self.pool.get_database_type() {
                    DatabaseType::Postgres => query.build_sqlx(PostgresQueryBuilder),
                    DatabaseType::Sqlite => query.build_sqlx(SqliteQueryBuilder),
                };

                let row = sqlx::query_with(&sql, values)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(|e| event::store::AppendError::Internal(e.into()))?;

                // Falls noch keine Events da sind, startet max() bei NULL -> 0
                row.try_get::<i32, _>(0).unwrap_or(0)
            }
        };

        // 2. Insert Statement aufbauen
        let mut insert = Query::insert();
        insert.into_table(Events::Table).columns([
            Events::EventId,
            Events::EventType,
            Events::AggregateType,
            Events::AggregateId,
            Events::AggregateVersion,
            Events::Data,
            Events::Metadata,
        ]);

        let mut final_version = current_version;

        for (i, event) in events.into_iter().enumerate() {
            final_version = current_version + (i as i32) + 1;
            let event_type = event.message.name();
            let serialized = self.serde.serialize(event.message).unwrap(); // Handle error
            let metadata_json = serde_json::to_value(&event.metadata)
                .expect("Failed to serialize metadata to JSON");

            insert.values_panic([
                Uuid::now_v7().into(), // UUID v7 generieren!
                event_type.into(),     // Aus dem Event holen
                self.aggregate_type.clone().into(),
                aggregate_id_uuid.into(),
                final_version.into(),
                serialized.into(),
                metadata_json.into(),
            ]);
        }

        let (sql, values) = match self.pool.get_database_type() {
            DatabaseType::Postgres => insert.build_sqlx(PostgresQueryBuilder),
            DatabaseType::Sqlite => insert.build_sqlx(SqliteQueryBuilder),
        };

        // 3. Ausführen und auf Unique-Konflikte (Optimistic Locking) prüfen
        match sqlx::query_with(&sql, values).execute(&mut *tx).await {
            Ok(_) => {
                tx.commit()
                    .await
                    .map_err(|e| event::store::AppendError::Internal(e.into()))?;
                Ok(final_version as Version)
            }
            Err(e) if is_conflict_error(&e) => {
                Err(event::store::AppendError::Conflict(
                    version::ConflictError {
                        expected: current_version as Version,
                        actual: final_version as Version, // Hier könnte man noch genauer prüfen
                    },
                ))
            }
            Err(e) => Err(event::store::AppendError::Internal(e.into())),
        }
    }
}
