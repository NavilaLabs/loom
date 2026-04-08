use eventually::aggregate;

use crate::tenant::activity::{
    self,
    domain::{
        aggregates::{Activity, ActivityId},
        events::ActivityEvent,
    },
};
use crate::tenant::project::ProjectId;

#[eventually_macros::aggregate_root(Activity)]
pub struct ActivityCommand;

impl ActivityCommand {
    pub fn create(
        &self,
        id: ActivityId,
        project_id: Option<ProjectId>,
        name: String,
    ) -> Result<Self, crate::Error> {
        Ok(
            aggregate::Root::<Activity>::record_new(
                ActivityEvent::Created { id, project_id, name }.into(),
            )
            .map_err(activity::DomainError::from)?
            .into(),
        )
    }

    pub fn update(
        &mut self,
        name: String,
        comment: Option<String>,
        visible: bool,
        billable: bool,
    ) -> Result<(), crate::Error> {
        self.record_that(
            ActivityEvent::Updated { name, comment, visible, billable }.into(),
        )
        .map_err(|e| activity::DomainError::AggregateError(e).into())
    }
}
