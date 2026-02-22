use std::{fmt::Display, str::FromStr};

use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Id(Uuid);

impl Id {
    /// Generates a new identifier.
    pub fn generate() -> Self {
        Self(Uuid::new_v7(uuid::Timestamp::now(uuid::ContextV7::new())))
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<Uuid> for Id {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl TryFrom<String> for Id {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(Uuid::from_str(&value)?))
    }
}

impl Default for Id {
    fn default() -> Self {
        Self(Uuid::new_v7(uuid::Timestamp::now(uuid::ContextV7::new())))
    }
}

pub type EventId = Id;

pub type AggregateType = String;

/// An identifier for an aggregate.
pub type AggregateId = Id;

pub type AggregateVersion = u64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityToken(Vec<u8>);

impl EntityToken {
    /// Generates a new entity token.
    pub fn generate() -> Self {
        let mut bytes = [0u8; 16];
        rand::rng().fill_bytes(&mut bytes);

        Self(bytes.to_vec())
    }
}

impl Default for EntityToken {
    fn default() -> Self {
        Self(vec![0u8; 16])
    }
}

impl AsRef<[u8]> for EntityToken {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Display for EntityToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

pub type CorrelationId = Id;
pub type CausationId = Id;

pub type UserId = Id;
