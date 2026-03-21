use crate::admin::user::domain::aggregates::User;
use eventually::aggregate::repository::{Getter, Saver};

pub trait UserRepository: Getter<User> + Saver<User> + Send + Sync {}
