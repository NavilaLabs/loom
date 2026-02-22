use modules::{
    AggregateMeta, EventContext, EventEnvelope, EventTimestamps, EventType, EventVersion,
    shared::value_objects::EventId,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value as JsonValue;

use crate::integrity::IntegrityChain;

pub mod config;
pub mod database;
pub mod event_bus;
pub mod event_store;
pub mod integrity;
pub mod projections;

pub trait ImplError {
    type Error: From<Error> + Send + Sync;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    DateTimeError(#[from] chrono::ParseError),
    #[error("{0}")]
    EnvVarError(#[from] dotenvy::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Url(#[from] url::ParseError),
    #[error("{0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("{0}")]
    JsonError(#[from] serde_json::Error),

    #[error("{0}")]
    ModulesError(#[from] modules::Error),
    #[error("{0}")]
    ConfigError(#[from] config::Error),
    #[error("{0}")]
    DatabaseError(#[from] database::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRecord {
    // Event Identity
    event_id: EventId,
    event_type: String,
    event_version: u8,

    #[serde(flatten)]
    aggregate: AggregateMeta,
    #[serde(flatten)]
    context: EventContext,
    #[serde(flatten)]
    timestamps: EventTimestamps,

    // Data payloads
    data: JsonValue,
    metadata: JsonValue,

    // Integrity
    hash: Vec<u8>,
    previous_hash: Vec<u8>,
}

impl EventRecord {
    pub fn new(
        event_id: EventId,
        event_type: String,
        event_version: u8,
        aggregate: AggregateMeta,
        context: EventContext,
        timestamps: EventTimestamps,
        data: JsonValue,
        metadata: JsonValue,
        hash: Vec<u8>,
        previous_hash: Vec<u8>,
    ) -> Self {
        Self {
            event_id,
            event_type,
            event_version,
            aggregate,
            context,
            timestamps,
            data,
            metadata,
            hash,
            previous_hash,
        }
    }

    pub fn get_event_id(&self) -> &EventId {
        &self.event_id
    }

    pub fn get_event_type(&self) -> &str {
        &self.event_type
    }

    pub fn get_event_version(&self) -> u8 {
        self.event_version
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

    pub fn get_data(&self) -> &JsonValue {
        &self.data
    }

    pub fn get_metadata(&self) -> &JsonValue {
        &self.metadata
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    pub fn get_previous_hash(&self) -> &[u8] {
        &self.previous_hash
    }
}

impl EventRecord {
    pub fn from_envelope<T>(
        envelope: EventEnvelope<T>,
        previous_hash: Vec<u8>,
    ) -> Result<Self, Error>
    where
        T: EventType + EventVersion + Serialize + Send + Sync,
    {
        let payload = envelope.get_payload();
        let data = serde_json::to_value(&payload)?;

        let mut record = EventRecord {
            event_id: envelope.get_event_id().clone(),
            event_type: payload.get_event_type().to_string(),
            event_version: <T as EventVersion>::VERSION,
            aggregate: envelope.get_aggregate().clone(),
            context: envelope.get_context().clone(),
            timestamps: envelope.get_timestamps().clone(),
            data,
            metadata: envelope.get_metadata().clone(),

            previous_hash: previous_hash.clone(),
            hash: Vec::new(),
        };
        record.hash = record.calculate_hash(&previous_hash);

        Ok(record)
    }
}

impl<T: DeserializeOwned> TryInto<EventEnvelope<T>> for EventRecord
where
    T: EventType + EventVersion + Serialize + Send + Sync,
{
    type Error = Error;

    fn try_into(self) -> Result<EventEnvelope<T>, Self::Error> {
        let payload = serde_json::from_value(self.data)?;

        Ok(EventEnvelope::new(
            self.event_id,
            self.aggregate,
            self.context,
            self.timestamps,
            payload,
            self.metadata,
        ))
    }
}
