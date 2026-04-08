use async_trait::async_trait;

use crate::tenant::activity_rate::domain::aggregates::ActivityRate;
use eventually::aggregate::repository::{Getter, Saver};

#[async_trait]
pub trait ActivityRateRepository: Getter<ActivityRate> + Saver<ActivityRate> + Send + Sync {}
