use async_trait::async_trait;
use eventually::{
    event,
    message::Message,
    serde,
    version::{self, Version},
};
use sea_orm::ExprTrait;
use sea_query::{Expr, PostgresQueryBuilder, Query, SqliteQueryBuilder};
use sea_query_sqlx::SqlxBinder;
use sqlx::{AnyPool, Row};
use uuid::Uuid;

use crate::infrastructure::{
    Pool, StateConnected,
    eventually::{DbBackend, Events, is_conflict_error},
};

pub struct EventStore<Scope, Id, Evt, Serde> {
    pool: Pool<StateConnected, Scope>, // sqlx::AnyPool erlaubt PG und SQLite
    backend: DbBackend,
    aggregate_type: String,
    serde: Serde,
    _id: std::marker::PhantomData<Id>,
    _evt: std::marker::PhantomData<Evt>,
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
        // Wandelt den UUID-String wieder in eine echte UUID für die DB um
        let aggregate_id_uuid = Uuid::parse_str(&aggregate_id).unwrap();

        let from_version = match select {
            event::VersionSelect::All => 0,
            event::VersionSelect::From(v) => v as i32,
        };

        let mut query = Query::select();
        query
            .columns([Events::AggregateVersion, Events::Data, Events::Metadata])
            .from(Events::Table)
            .and_where(Expr::col(Events::AggregateType).eq(self.aggregate_type.clone()))
            .and_where(Expr::col(Events::AggregateId).eq(aggregate_id_uuid))
            .and_where(Expr::col(Events::AggregateVersion).gte(from_version))
            .order_by(Events::AggregateVersion, sea_query::Order::Asc);

        let (sql, values) = match self.backend {
            DbBackend::Postgres => query.build_sqlx(PostgresQueryBuilder),
            DbBackend::Sqlite => query.build_sqlx(SqliteQueryBuilder),
        };

        // Hier würdest du sqlx::query_with aufrufen und den Stream zurückgeben
        // (Pseudocode für den Stream, analog zu deinem alten Code)
        todo!("Execute mit self.pool und mappen in event::Persisted")
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

                let (sql, values) = match self.backend {
                    DbBackend::Postgres => query.build_sqlx(PostgresQueryBuilder),
                    DbBackend::Sqlite => query.build_sqlx(SqliteQueryBuilder),
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
            let serialized = self.serde.serialize(event.message).unwrap(); // Handle error in prod
            // let metadata_json = sqlx::types::Json(event.metadata);
            let metadata_json = serde_json::to_value(&event.metadata)
                .expect("Failed to serialize metadata to JSON");

            insert.values_panic([
                Uuid::now_v7().into(), // UUID v7 generieren!
                "MyEventType".into(),  // Aus dem Event holen
                self.aggregate_type.clone().into(),
                aggregate_id_uuid.into(),
                final_version.into(),
                serialized.into(),
                metadata_json.into(),
            ]);
        }

        let (sql, values) = match self.backend {
            DbBackend::Postgres => insert.build_sqlx(PostgresQueryBuilder),
            DbBackend::Sqlite => insert.build_sqlx(SqliteQueryBuilder),
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
