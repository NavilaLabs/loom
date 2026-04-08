use async_trait::async_trait;

use crate::tenant::project_rate::domain::aggregates::ProjectRate;
use eventually::aggregate::repository::{Getter, Saver};

#[async_trait]
pub trait ProjectRateRepository: Getter<ProjectRate> + Saver<ProjectRate> + Send + Sync {}
