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

#[derive(Debug, thiserror::Error)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_id() -> UserId {
        "019d0ce8-facb-7c90-b9d7-287ae4f17c91"
            .parse()
            .expect("valid UUID")
    }

    #[test]
    fn apply_created_event_to_no_state_builds_user() {
        let id = test_id();
        let event = UserEvent::Created {
            id: id.clone(),
            name: "Alice".to_string(),
        };
        let result = User::apply(None, event);
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id(), &id);
        assert_eq!(user.name(), "Alice");
    }

    #[test]
    fn apply_created_event_to_existing_user_returns_already_exists() {
        let id = test_id();
        let existing = User {
            id: id.clone(),
            name: "Alice".to_string(),
        };
        let event = UserEvent::Created {
            id: id.clone(),
            name: "Bob".to_string(),
        };
        let result = User::apply(Some(existing), event);
        assert!(matches!(result, Err(Error::AlreadyExists)));
    }
}
