use eventually::aggregate;

use crate::tenant::tag::{
    self,
    domain::{
        aggregates::{Tag, TagId},
        events::TagEvent,
    },
};
use crate::tenant::timesheet::TimesheetId;

#[eventually_macros::aggregate_root(Tag)]
pub struct TagCommand;

impl TagCommand {
    /// # Errors
    ///
    /// Returns an error if the domain event cannot be applied to the aggregate.
    pub fn create(&self, id: TagId, name: String) -> Result<Self, crate::Error> {
        Ok(
            aggregate::Root::<Tag>::record_new(TagEvent::Created { id, name }.into())
                .map_err(tag::DomainError::from)?
                .into(),
        )
    }

    /// # Errors
    ///
    /// Returns an error if the domain event cannot be applied to the aggregate.
    pub fn rename(&mut self, name: String) -> Result<(), crate::Error> {
        self.record_that(TagEvent::Renamed { name }.into())
            .map_err(|e| tag::DomainError::AggregateError(e).into())
    }

    /// # Errors
    ///
    /// Returns an error if the domain event cannot be applied to the aggregate.
    pub fn tag_timesheet(&mut self, timesheet_id: TimesheetId) -> Result<(), crate::Error> {
        self.record_that(TagEvent::TimesheetTagged { timesheet_id }.into())
            .map_err(|e| tag::DomainError::AggregateError(e).into())
    }

    /// # Errors
    ///
    /// Returns an error if the domain event cannot be applied to the aggregate.
    pub fn untag_timesheet(&mut self, timesheet_id: TimesheetId) -> Result<(), crate::Error> {
        self.record_that(TagEvent::TimesheetUntagged { timesheet_id }.into())
            .map_err(|e| tag::DomainError::AggregateError(e).into())
    }
}
