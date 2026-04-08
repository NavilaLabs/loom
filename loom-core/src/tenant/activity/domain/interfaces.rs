use async_trait::async_trait;

use crate::tenant::activity::domain::aggregates::Activity;
use eventually::aggregate::repository::{Getter, Saver};

#[async_trait]
pub trait ActivityRepository: Getter<Activity> + Saver<Activity> + Send + Sync {}
