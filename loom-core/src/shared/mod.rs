use std::ops::Deref;

use uuid::{Timestamp, Uuid};

#[derive(Debug, Clone)]
pub struct Id(Uuid);

impl Id {
    pub fn new(timestamp: Timestamp) -> Self {
        Id(Uuid::new_v7(timestamp))
    }
}

impl From<Id> for Uuid {
    fn from(id: Id) -> Self {
        id.0
    }
}

impl From<Uuid> for Id {
    fn from(uuid: Uuid) -> Self {
        Id(uuid)
    }
}

impl From<Id> for String {
    fn from(id: Id) -> Self {
        id.0.to_string()
    }
}

impl TryFrom<String> for Id {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Uuid::parse_str(&value)?.into())
    }
}

impl Deref for Id {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Uuid> for Id {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}
