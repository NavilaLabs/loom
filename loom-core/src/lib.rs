use std::{fmt::Display, str::FromStr};

use eventually::aggregate::repository;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod admin;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("missing field: {0}")]
    MissingField(String),

    #[error("{0:?}")]
    AdminDatabaseError(#[from] admin::Error),
    #[error("{0:?}")]
    DatabaseSaveError(#[from] repository::SaveError),
    #[error("{0:?}")]
    DatabaseGetError(#[from] repository::GetError),
    #[error("{0:?}")]
    ParseUuidError(#[from] uuid::Error),
    #[error("{0:?}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("{0:?}")]
    SqlxError(#[from] sqlx::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AggregateId(pub Uuid);

impl Display for AggregateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for AggregateId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl FromStr for AggregateId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(Self)
    }
}
