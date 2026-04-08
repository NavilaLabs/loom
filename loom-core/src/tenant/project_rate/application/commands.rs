use eventually::aggregate;

use crate::tenant::project::ProjectId;
use crate::tenant::project_rate::{
    self,
    domain::{
        aggregates::{ProjectRate, ProjectRateId},
        events::{ProjectRateEvent, UserId},
    },
};

#[eventually_macros::aggregate_root(ProjectRate)]
pub struct ProjectRateCommand;

impl ProjectRateCommand {
    pub fn set(
        &self,
        id: ProjectRateId,
        project_id: ProjectId,
        user_id: Option<UserId>,
        hourly_rate: i64,
        internal_rate: Option<i64>,
    ) -> Result<Self, crate::Error> {
        Ok(
            aggregate::Root::<ProjectRate>::record_new(
                ProjectRateEvent::Set { id, project_id, user_id, hourly_rate, internal_rate }.into(),
            )
            .map_err(project_rate::DomainError::from)?
            .into(),
        )
    }

    pub fn remove(&mut self) -> Result<(), crate::Error> {
        self.record_that(ProjectRateEvent::Removed.into())
            .map_err(|e| project_rate::DomainError::AggregateError(e).into())
    }
}
