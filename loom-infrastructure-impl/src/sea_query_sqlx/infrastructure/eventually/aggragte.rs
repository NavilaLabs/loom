use async_trait::async_trait;
use eventually::{
    aggregate::{self, Aggregate},
    serde,
    version::Version,
};
use sea_orm::ExprTrait;
use sea_query::{Expr, OnConflict, PostgresQueryBuilder, Query, SqliteQueryBuilder};
use sea_query_sqlx::SqlxBinder;
use sqlx::{AnyPool, Row};
use uuid::Uuid;

use crate::infrastructure::eventually::{DbBackend, Snapshots};

pub struct Repository<T: Aggregate, Serde, EvtSerde> {
    pool: AnyPool,
    backend: DbBackend,
    aggregate_serde: Serde,
    event_serde: EvtSerde,
    _t: std::marker::PhantomData<T>,
}

#[async_trait]
impl<T, Serde, EvtSerde> aggregate::repository::Getter<T> for Repository<T, Serde, EvtSerde>
where
    T: Aggregate + Send + Sync,
    <T as Aggregate>::Id: ToString,
    Serde: serde::Serde<T> + Send + Sync,
    EvtSerde: serde::Serde<T::Event> + Send + Sync,
{
    async fn get(&self, id: &T::Id) -> Result<aggregate::Root<T>, aggregate::repository::GetError> {
        let aggregate_id_uuid = Uuid::parse_str(&id.to_string()).unwrap();

        // 1. Hole Snapshot
        let mut query = Query::select();
        query
            .columns([Snapshots::AggregateVersion, Snapshots::State])
            .from(Snapshots::Table)
            .and_where(Expr::col(Snapshots::AggregateType).eq(T::type_name()))
            .and_where(Expr::col(Snapshots::AggregateId).eq(aggregate_id_uuid));

        let (sql, values) = match self.backend {
            DbBackend::Postgres => query.build_sqlx(PostgresQueryBuilder),
            DbBackend::Sqlite => query.build_sqlx(SqliteQueryBuilder),
        };

        let row = sqlx::query_with(&sql, values)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        if let Some(r) = row {
            let version: i32 = r.try_get("aggregate_version").unwrap();
            let state: Vec<u8> = r.try_get("state").unwrap();
            let aggregate: T = self.aggregate_serde.deserialize(&state).unwrap();

            // TODO: Jetzt die Events vom EventStore ab Version > `version` laden
            // und via `root.apply(event)` falten!
            Ok(aggregate::Root::rehydrate_from_state(
                version as Version,
                aggregate,
            ))
        } else {
            // Kein Snapshot da -> Lade Events ab Version 0
            Err(aggregate::repository::GetError::NotFound)
        }
    }
}

#[async_trait]
impl<T, Serde, EvtSerde> aggregate::repository::Saver<T> for Repository<T, Serde, EvtSerde>
where
    T: Aggregate + Send + Sync,
    <T as Aggregate>::Id: ToString,
    Serde: serde::Serde<T> + Send + Sync,
    EvtSerde: serde::Serde<T::Event> + Send + Sync,
{
    async fn save(
        &self,
        root: &mut aggregate::Root<T>,
    ) -> Result<(), aggregate::repository::SaveError> {
        let events = root.take_uncommitted_events();
        if events.is_empty() {
            return Ok(());
        }

        let aggregate_id_uuid = Uuid::parse_str(&root.aggregate_id().to_string()).unwrap();
        let current_version = root.version() as i32;

        // 1. Events via EventStore (Appender) speichern (wie oben definiert)
        // ... (Ruf hier die Logik aus deinem EventStore auf)

        // 2. Snapshot Upsert (Der elegante Teil mit sea-query)
        // Man macht das meistens nur alle X Events (z.B. current_version % 50 == 0),
        // hier machen wir es beispielhaft immer.
        let out_state = root.to_aggregate_type::<T>();
        let bytes_state = self.aggregate_serde.serialize(out_state).unwrap();

        let mut insert = Query::insert();
        insert
            .into_table(Snapshots::Table)
            .columns([
                Snapshots::AggregateType,
                Snapshots::AggregateId,
                Snapshots::AggregateVersion,
                Snapshots::State,
            ])
            .values_panic([
                T::type_name().into(),
                aggregate_id_uuid.into(),
                current_version.into(),
                bytes_state.into(),
            ])
            .on_conflict(
                // Das ist das Geheimnis für Upsert in SQLite UND Postgres
                OnConflict::columns([Snapshots::AggregateType, Snapshots::AggregateId])
                    .update_columns([
                        Snapshots::AggregateVersion,
                        Snapshots::State,
                        Snapshots::UpdatedAt,
                    ])
                    .to_owned(),
            );

        let (sql, values) = match self.backend {
            DbBackend::Postgres => insert.build_sqlx(PostgresQueryBuilder),
            DbBackend::Sqlite => insert.build_sqlx(SqliteQueryBuilder),
        };

        sqlx::query_with(&sql, values)
            .execute(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!(e))?; // todo: implement error handling

        Ok(())
    }
}
