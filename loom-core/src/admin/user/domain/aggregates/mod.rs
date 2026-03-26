use eventually::aggregate::Aggregate;
use serde::{Deserialize, Serialize};

use crate::{admin::user::UserEvent, shared::AggregateId};

pub type UserId = AggregateId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    name: String,
}

impl User {
    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("user already exists")]
    AlreadyExists,
}

impl Aggregate for User {
    type Id = UserId;
    type Event = UserEvent;
    type Error = Error;

    fn type_name() -> &'static str {
        "user"
    }

    fn aggregate_id(&self) -> &Self::Id {
        &self.id
    }

    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
        match (state, event) {
            (None, UserEvent::Created { id, name }) => Ok(Self { id, name }),
            (Some(_), UserEvent::Created { .. }) => Err(Error::AlreadyExists),
        }
    }
}
