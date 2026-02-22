mod database;
mod event_store;
mod provider;

use modules::{
    AggregateMeta, EventContext, EventTimestamps,
    shared::value_objects::{AggregateId, CausationId, CorrelationId, EventId, UserId},
};
pub use provider::*;
use sqlx::{Row, any::AnyRow};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SQLx error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("SeaORM error: {0}")]
    SeaOrmError(#[from] sea_orm::DbErr),
    #[error("SeaQuery error: {0}")]
    SeaQueryError(#[from] sea_query::error::Error),
    #[error("{0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),
    #[error("Unsupported database type: {0}")]
    UnsupportedDatabaseType(String),
}

impl From<sqlx::migrate::MigrateError> for crate::Error {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        Error::MigrateError(err).into()
    }
}

impl From<sqlx::Error> for crate::Error {
    fn from(err: sqlx::Error) -> Self {
        Error::SqlxError(err).into()
    }
}

impl From<sea_orm::DbErr> for crate::Error {
    fn from(err: sea_orm::DbErr) -> Self {
        Error::SeaOrmError(err).into()
    }
}

impl From<sea_query::error::Error> for crate::Error {
    fn from(err: sea_query::error::Error) -> Self {
        Error::SeaQueryError(err).into()
    }
}

struct EventRecord(infrastructure::EventRecord);

impl TryFrom<AnyRow> for EventRecord {
    type Error = crate::Error;

    fn try_from(row: AnyRow) -> Result<EventRecord, Self::Error> {
        let event_id: String = row.try_get("event_id")?;
        let event_id = EventId::try_from(event_id)?;
        let event_type: String = row.try_get("event_type")?;
        let event_version: i64 = row.try_get("event_version")?;

        let aggregate_type: String = row.try_get("aggregate_type")?;
        let aggregate_id: String = row.try_get("aggregate_id")?;
        let aggregate_id = AggregateId::try_from(aggregate_id)?;
        let aggregate_version: i32 = row.try_get("aggregate_version")?;

        let correlation_id: Option<String> = row.try_get("correlation_id")?;
        let correlation_id = match correlation_id {
            Some(id) => Some(CorrelationId::try_from(id)?),
            None => None,
        };
        let causation_id: Option<String> = row.try_get("causation_id")?;
        let causation_id = match causation_id {
            Some(id) => Some(CausationId::try_from(id)?),
            None => None,
        };
        let created_by: String = row.try_get("created_by")?;
        let created_by = UserId::try_from(created_by)?;
        let owned_by: Option<String> = row.try_get("owned_by")?;
        let owned_by = match owned_by {
            Some(id) => Some(UserId::try_from(id)?),
            None => None,
        };

        let created_at: String = row.try_get("created_at")?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at)?.to_utc();
        let effective_at: Option<String> = row.try_get("effective_at")?;
        let effective_at = match effective_at {
            Some(effective_at) => {
                Some(chrono::DateTime::parse_from_rfc3339(&effective_at)?.to_utc())
            }
            None => None,
        };

        let data: String = row.try_get("data")?;
        let data: serde_json::Value = serde_json::from_str(&data)?;
        let metadata: String = row.try_get("metadata")?;
        let metadata = serde_json::from_str(&metadata)?;

        let hash: Vec<u8> = row.try_get("hash")?;
        let previous_hash: Vec<u8> = row.try_get("previous_hash")?;

        Ok(EventRecord(infrastructure::EventRecord::new(
            event_id,
            event_type,
            event_version.try_into().unwrap(), // TODO!
            AggregateMeta::new(
                aggregate_type,
                aggregate_id,
                aggregate_version.try_into().unwrap(),
            ),
            EventContext::new(correlation_id, causation_id, created_by, owned_by),
            EventTimestamps::new(created_at, effective_at),
            data,
            metadata,
            hash,
            previous_hash,
        )))
    }
}
