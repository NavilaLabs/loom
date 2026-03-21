use eventually::aggregate;

use crate::admin::user::{
    aggregates::{Error, User, UserId},
    events::UserEvent,
};

#[eventually_macros::aggregate_root(User)]
#[derive(Debug, Clone, PartialEq)]
pub struct UserRoot;

impl UserRoot {
    pub fn create(id: UserId, name: String) -> Result<Self, Error> {
        Ok(aggregate::Root::<User>::record_new(UserEvent::Created { id, name }.into())?.into())
    }
}
