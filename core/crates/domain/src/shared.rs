use rand::{Rng, distr::Alphanumeric};
use uuid::Uuid;

pub struct AggregateId(Uuid);

impl From<Uuid> for AggregateId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl AsRef<Uuid> for AggregateId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

pub struct EntityToken(String);

impl EntityToken {
    pub fn new() -> Self {
        Self(
            rand::rng()
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect(),
        )
    }
}

impl From<String> for EntityToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for EntityToken {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl AsRef<str> for EntityToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
