use crate::admin::user::domain::{aggregates::User, events::UserEvent, interfaces};
use eventually::serde::Json;
use eventually_any::snapshot::Repository;

pub type UserRepository = Repository<User, Json<User>, Json<UserEvent>>;

impl interfaces::UserRepository for UserRepository {}
