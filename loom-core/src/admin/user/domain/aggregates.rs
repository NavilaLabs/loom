use eventually::aggregate::Aggregate;
use serde::{Deserialize, Serialize};

use crate::{admin::user::UserEvent, shared::AggregateId};

pub type UserId = AggregateId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    name: String,
    email: String,
    password_hash: String,
}

impl User {
    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
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
            (
                None,
                UserEvent::Created {
                    id,
                    name,
                    email,
                    password_hash,
                },
            ) => Ok(Self {
                id,
                name,
                email,
                password_hash,
            }),
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

    fn created_event(id: UserId, name: &str) -> UserEvent {
        UserEvent::Created {
            id,
            name: name.to_string(),
            email: "alice@example.com".to_string(),
            password_hash: "$2b$12$hash".to_string(),
        }
    }

    #[test]
    fn apply_created_event_to_no_state_builds_user() {
        let id = test_id();
        let event = created_event(id.clone(), "Alice");
        let result = User::apply(None, event);
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id(), &id);
        assert_eq!(user.name(), "Alice");
        assert_eq!(user.email(), "alice@example.com");
    }

    #[test]
    fn apply_created_event_to_existing_user_returns_already_exists() {
        let id = test_id();
        let existing = User::apply(None, created_event(id.clone(), "Alice")).unwrap();
        let result = User::apply(Some(existing), created_event(id, "Bob"));
        assert!(matches!(result, Err(Error::AlreadyExists)));
    }
}
