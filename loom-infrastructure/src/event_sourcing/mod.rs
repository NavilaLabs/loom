use async_trait::async_trait;
use chrono::{DateTime, Utc};
use loom_shared::event_sourcing::{Aggregate, Projection};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value as JsonValue;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventRow {
    global_position: u64,
    event_stream_id: Uuid,
    event_id: Uuid,
    event_type: String,
    event_schema_version: u32,
    aggregate_type: String,
    aggregate_id: Uuid,
    aggregate_version: u32,
    aggreagte_schema_version: u32,
    data: Vec<u8>,
    metadata: Option<Vec<u8>>,
    created_at: DateTime<Utc>,
    effective_at: Option<DateTime<Utc>>,
    correlation_id: Option<Uuid>,
    causation_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventStreamRow {
    id: Uuid,
    version: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotRow {
    event_stream_id: Uuid,
    id: Uuid,
    aggregate_type: String,
    aggregate_id: Uuid,
    aggregate_version: u64,
    data: Vec<u8>,
    metadata: Option<Vec<u8>>,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<T> {
    event_type: String,
    event_schema_version: u32,
    aggregate_type: String,
    aggregate_id: Uuid,
    aggregate_version: u32,
    aggreagte_schema_version: u32,
    data: T,
    metadata: Option<JsonValue>,
    created_at: DateTime<Utc>,
    effective_at: Option<DateTime<Utc>>,
    correlation_id: Option<Uuid>,
    causation_id: Option<Uuid>,
}

pub struct Envelope<T> {
    data: T,
    metadata: Metadata,
}

pub struct Metadata {
    correlation_id: Option<Uuid>,
    causation_id: Option<Uuid>,
}

#[async_trait]
pub trait Appender {
    async fn append_events<T: Aggregate>(
        &self,
        events: Vec<T>,
        metadata: Option<JsonValue>,
    ) -> Result<(), crate::Error>;
}

#[async_trait]
pub trait Streamer {
    /// Streams events for the given aggregate, optionally filtered by version.
    async fn stream<T>(
        &self,
        aggregate_id: Uuid,
        version: Option<u64>,
    ) -> Result<Vec<EventRow>, crate::Error>;
}

#[async_trait]
pub trait Snapshotter {
    async fn snapshot<T>(&self, aggregate_id: Uuid) -> Result<(), crate::Error>;
}

#[async_trait]
pub trait Projector {
    async fn project<T, U: Projection>(
        &self,
        transaction: &T,
        event: U,
    ) -> Result<(), crate::Error>;
}
