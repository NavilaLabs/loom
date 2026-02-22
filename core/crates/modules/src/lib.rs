use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::shared::value_objects::{
    AggregateId, AggregateType, AggregateVersion, CausationId, CorrelationId, EventId, UserId,
};

pub mod shared;
pub mod tenant;
pub mod user;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    UuidError(#[from] uuid::Error),
    #[error("{0}")]
    HexError(#[from] hex::FromHexError),
}

pub trait EventType {
    fn get_event_type(&self) -> &str;
}

pub trait EventVersion {
    const VERSION: u8;
}

/// Represents the aggregate this event belongs to.
/// Useful for enforcing optimistic concurrency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateMeta {
    #[serde(rename = "aggregate_type")]
    r#type: AggregateType,
    #[serde(rename = "aggregate_id")]
    id: AggregateId,
    #[serde(rename = "aggregate_version")]
    version: AggregateVersion,
}

impl AggregateMeta {
    pub fn new(
        aggregate_type: AggregateType,
        aggregate_id: AggregateId,
        aggregate_version: AggregateVersion,
    ) -> Self {
        AggregateMeta {
            r#type: aggregate_type,
            id: aggregate_id,
            version: aggregate_version,
        }
    }

    pub fn get_type(&self) -> &AggregateType {
        &self.r#type
    }

    pub fn get_id(&self) -> &AggregateId {
        &self.id
    }

    pub fn get_version(&self) -> AggregateVersion {
        self.version
    }
}

impl Default for AggregateMeta {
    fn default() -> Self {
        AggregateMeta {
            r#type: AggregateType::default(),
            id: AggregateId::default(),
            version: AggregateVersion::default(),
        }
    }
}

/// Tracing information for the Event Store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventContext {
    correlation_id: Option<CorrelationId>,
    causation_id: Option<CausationId>,
    created_by: UserId,
    owned_by: Option<UserId>,
}

impl EventContext {
    pub fn new(
        correlation_id: Option<CorrelationId>,
        causation_id: Option<CausationId>,
        created_by: UserId,
        owned_by: Option<UserId>,
    ) -> Self {
        EventContext {
            correlation_id,
            causation_id,
            created_by,
            owned_by,
        }
    }

    pub fn get_correlation_id(&self) -> Option<&CorrelationId> {
        self.correlation_id.as_ref()
    }

    pub fn get_causation_id(&self) -> Option<&CausationId> {
        self.causation_id.as_ref()
    }

    pub fn get_created_by(&self) -> &UserId {
        &self.created_by
    }

    pub fn get_owned_by(&self) -> Option<&UserId> {
        self.owned_by.as_ref()
    }
}

impl Default for EventContext {
    fn default() -> Self {
        EventContext {
            correlation_id: None,
            causation_id: None,
            created_by: UserId::default(),
            owned_by: None,
        }
    }
}

/// Timing information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTimestamps {
    created_at: DateTime<Utc>,
    effective_at: Option<DateTime<Utc>>,
}

impl Default for EventTimestamps {
    fn default() -> Self {
        EventTimestamps {
            created_at: Utc::now(),
            effective_at: None,
        }
    }
}

impl EventTimestamps {
    pub fn new(created_at: DateTime<Utc>, effective_at: Option<DateTime<Utc>>) -> Self {
        EventTimestamps {
            created_at,
            effective_at,
        }
    }

    pub fn get_created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn get_effective_at(&self) -> Option<&DateTime<Utc>> {
        self.effective_at.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T>
where
    T: Send + Sync,
{
    event_id: EventId,
    aggregate: AggregateMeta,
    context: EventContext,
    timestamps: EventTimestamps,
    payload: T, // The actual domain event (e.g., UserCreated)
    metadata: JsonValue,
}

impl<T> EventEnvelope<T>
where
    T: Send + Sync,
{
    pub fn new(
        event_id: EventId,
        aggregate: AggregateMeta,
        context: EventContext,
        timestamps: EventTimestamps,
        payload: T,
        metadata: JsonValue,
    ) -> Self {
        EventEnvelope {
            event_id,
            aggregate,
            context,
            timestamps,
            payload,
            metadata,
        }
    }

    pub fn get_event_id(&self) -> &EventId {
        &self.event_id
    }

    pub fn get_aggregate(&self) -> &AggregateMeta {
        &self.aggregate
    }

    pub fn get_context(&self) -> &EventContext {
        &self.context
    }

    pub fn get_timestamps(&self) -> &EventTimestamps {
        &self.timestamps
    }

    pub fn get_payload(&self) -> &T {
        &self.payload
    }

    pub fn get_metadata(&self) -> &JsonValue {
        &self.metadata
    }
}

#[cfg(test)]
impl<T> EventEnvelope<T>
where
    T: Send + Sync,
{
    pub fn new_test(
        event_id: EventId,
        aggregate: AggregateMeta,
        context: EventContext,
        timestamps: EventTimestamps,
        payload: T,
        metadata: JsonValue,
    ) -> Self {
        EventEnvelope {
            event_id,
            aggregate,
            context,
            timestamps,
            payload,
            metadata,
        }
    }
}
