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
