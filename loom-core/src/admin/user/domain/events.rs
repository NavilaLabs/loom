use eventually::message::Message;
use serde::{Deserialize, Serialize};

use crate::admin::user::UserId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserEvent {
    Created {
        id: UserId,
        name: String,
        email: String,
        password: String,
    },
}

impl Message for UserEvent {
    fn name(&self) -> &'static str {
        match self {
            UserEvent::Created { .. } => "UserCreated",
        }
    }
}
