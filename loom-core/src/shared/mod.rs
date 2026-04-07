use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AggregateId(pub Uuid);

impl AggregateId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for AggregateId {
    fn default() -> Self {
        Self::new()
    }
}

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
