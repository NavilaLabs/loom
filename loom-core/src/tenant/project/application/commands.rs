use eventually::aggregate;

use crate::tenant::project::{
    self,
    domain::{
        aggregates::{Project, ProjectId},
        events::ProjectEvent,
    },
};
use crate::tenant::customer::CustomerId;

#[eventually_macros::aggregate_root(Project)]
pub struct ProjectCommand;

impl ProjectCommand {
    pub fn create(
        &self,
        id: ProjectId,
        customer_id: CustomerId,
        name: String,
    ) -> Result<Self, crate::Error> {
        Ok(
            aggregate::Root::<Project>::record_new(
                ProjectEvent::Created { id, customer_id, name }.into(),
            )
            .map_err(project::DomainError::from)?
            .into(),
        )
    }

    pub fn update(
        &mut self,
        name: String,
        comment: Option<String>,
        order_number: Option<String>,
        visible: bool,
        billable: bool,
    ) -> Result<(), crate::Error> {
        self.record_that(
            ProjectEvent::Updated { name, comment, order_number, visible, billable }.into(),
        )
        .map_err(|e| project::DomainError::AggregateError(e).into())
    }

    pub fn set_budget(
        &mut self,
        time_budget: Option<i32>,
        money_budget: Option<i64>,
        budget_is_monthly: bool,
    ) -> Result<(), crate::Error> {
        self.record_that(
            ProjectEvent::BudgetUpdated { time_budget, money_budget, budget_is_monthly }.into(),
        )
        .map_err(|e| project::DomainError::AggregateError(e).into())
    }
}
