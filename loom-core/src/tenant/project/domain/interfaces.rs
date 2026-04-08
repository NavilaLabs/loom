use async_trait::async_trait;

use crate::tenant::project::domain::aggregates::Project;
use eventually::aggregate::repository::{Getter, Saver};

#[async_trait]
pub trait ProjectRepository: Getter<Project> + Saver<Project> + Send + Sync {}
