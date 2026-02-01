use std::{borrow::Cow, marker::PhantomData};

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::shared::value_objects::AggregateId;

pub mod shared;
pub mod tenant;
pub mod user;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid UUID\n  Found: {0}")]
    UuidError(#[from] uuid::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedEvent<'a, AggType, Data> {
    id: Uuid,

    #[serde(flatten)]
    aggregate_context: AggregateContext<'a, AggType>,

    payload: Data,

    event_context: EventContext,

    created_at: chrono::DateTime<chrono::Utc>,
    effective_at: chrono::DateTime<chrono::Utc>,

    hash: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AggregateContext<'a, Type> {
    id: AggregateId,
    r#type: Cow<'a, str>,
    version: u32,
    _type: PhantomData<Type>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventContext {
    correlation_id: Option<Uuid>,
    causation_id: Option<Uuid>,
    created_by: Uuid,
    owned_by: Option<Uuid>,
    metadata: Option<serde_json::Value>,
}

pub trait DomainEvent {
    fn get_event_type(&self) -> &'static str;
    fn get_event_version(&self) -> i32;
}
