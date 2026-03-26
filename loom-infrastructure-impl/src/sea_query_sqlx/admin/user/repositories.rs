use eventually_any::snapshot::Repository;
use loom_core::admin::user::{User, UserEvent};
use sqlx::types::Json;

pub type UserRepository = Repository<User, Json<User>, Json<UserEvent>>;
