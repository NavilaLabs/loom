use eventually::aggregate;

use crate::tenant::activity::ActivityId;
use crate::tenant::project::ProjectId;
use crate::tenant::timesheet::{
    self,
    domain::{
        aggregates::{Timesheet, TimesheetId},
        events::{TimesheetEvent, UserId},
    },
};

#[eventually_macros::aggregate_root(Timesheet)]
pub struct TimesheetCommand;

impl TimesheetCommand {
    /// # Errors
    ///
    /// Returns an error if the domain event cannot be applied to the aggregate.
    #[allow(clippy::too_many_arguments)]
    pub fn start(
        &self,
        id: TimesheetId,
        user_id: UserId,
        project_id: Option<ProjectId>,
        activity_id: Option<ActivityId>,
        start_time: String,
        timezone: String,
        billable: bool,
    ) -> Result<Self, crate::Error> {
        Ok(aggregate::Root::<Timesheet>::record_new(
            TimesheetEvent::Started {
                id,
                user_id,
                project_id,
                activity_id,
                start_time,
                timezone,
                billable,
            }
            .into(),
        )
        .map_err(timesheet::DomainError::from)?
        .into())
    }

    /// # Errors
    ///
    /// Returns an error if the domain event cannot be applied to the aggregate.
    #[allow(clippy::too_many_arguments)]
    pub fn stop(
        &mut self,
        end_time: String,
        duration: i32,
        hourly_rate: Option<i64>,
        fixed_rate: Option<i64>,
        internal_rate: Option<i64>,
        rate: Option<i64>,
    ) -> Result<(), crate::Error> {
        self.record_that(
            TimesheetEvent::Stopped {
                end_time,
                duration,
                hourly_rate,
                fixed_rate,
                internal_rate,
                rate,
            }
            .into(),
        )
        .map_err(|e| timesheet::DomainError::AggregateError(e).into())
    }

    /// # Errors
    ///
    /// Returns an error if the domain event cannot be applied to the aggregate.
    pub fn update(
        &mut self,
        description: Option<String>,
        billable: bool,
    ) -> Result<(), crate::Error> {
        self.record_that(
            TimesheetEvent::Updated {
                description,
                billable,
            }
            .into(),
        )
        .map_err(|e| timesheet::DomainError::AggregateError(e).into())
    }

    /// # Errors
    ///
    /// Returns an error if the domain event cannot be applied to the aggregate.
    pub fn reassign(
        &mut self,
        project_id: ProjectId,
        activity_id: ActivityId,
    ) -> Result<(), crate::Error> {
        self.record_that(
            TimesheetEvent::Reassigned {
                project_id,
                activity_id,
            }
            .into(),
        )
        .map_err(|e| timesheet::DomainError::AggregateError(e).into())
    }

    /// # Errors
    ///
    /// Returns an error if the domain event cannot be applied to the aggregate.
    pub fn export(&mut self) -> Result<(), crate::Error> {
        self.record_that(TimesheetEvent::Exported.into())
            .map_err(|e| timesheet::DomainError::AggregateError(e).into())
    }
}
