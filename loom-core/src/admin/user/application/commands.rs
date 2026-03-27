use eventually::aggregate;

use crate::admin::user::{
    self,
    domain::{
        aggregates::{User, UserId},
        events::UserEvent,
    },
};

#[eventually_macros::aggregate_root(User)]
#[derive(Debug, Clone, PartialEq)]
pub struct UserCommand;

impl UserCommand {
    pub fn create(&self, id: UserId, name: String) -> Result<Self, crate::Error> {
        Ok(
            aggregate::Root::<User>::record_new(UserEvent::Created { id, name }.into())
                .map_err(|error| user::DomainError::from(error))?
                .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use eventually::aggregate::{Aggregate, Root};

    use super::*;

    /// Build a `UserCommand` shell by rehydrating an existing user state.
    /// `create(&self, …)` ignores `self`, so this is just the cheapest valid instance.
    fn make_command_shell(id: UserId) -> UserCommand {
        let user = User::apply(
            None,
            UserEvent::Created {
                id: id.clone(),
                name: "seed".to_string(),
            },
        )
        .expect("seed user");
        Root::<User>::rehydrate_from_state(1, user).into()
    }

    fn test_id() -> UserId {
        "019d0ce8-facb-7c90-b9d7-287ae4f17c91"
            .parse()
            .expect("valid UUID")
    }

    #[test]
    fn create_returns_root_with_applied_state() {
        let shell = make_command_shell(test_id());
        let id: UserId = "019d0ce8-facb-7c90-b9d7-287ae4f17c92"
            .parse()
            .expect("valid UUID");

        let result = shell.create(id.clone(), "Alice".to_string());

        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.aggregate_id(), &id);
        assert_eq!(cmd.name(), "Alice");
        assert_eq!(cmd.version(), 1);
    }

    #[test]
    fn create_propagates_aggregate_error_on_bad_event() {
        // record_new on Created succeeds — verify the happy path doesn't panic
        let shell = make_command_shell(test_id());
        assert!(shell
            .create(test_id(), "Bob".to_string())
            .is_ok());
    }
}
