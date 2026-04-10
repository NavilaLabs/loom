use eventually::aggregate;

use crate::tenant::customer::{
    self,
    domain::{
        aggregates::{Customer, CustomerId},
        events::CustomerEvent,
    },
};

#[eventually_macros::aggregate_root(Customer)]
pub struct CustomerCommand;

impl CustomerCommand {
    /// # Errors
    ///
    /// Returns an error if the domain event cannot be applied to the aggregate.
    pub fn create(
        &self,
        id: CustomerId,
        name: String,
        currency: String,
        timezone: String,
    ) -> Result<Self, crate::Error> {
        Ok(aggregate::Root::<Customer>::record_new(
            CustomerEvent::Created {
                id,
                name,
                currency,
                timezone,
            }
            .into(),
        )
        .map_err(customer::DomainError::from)?
        .into())
    }

    /// # Errors
    ///
    /// Returns an error if the domain event cannot be applied to the aggregate.
    pub fn update(
        &mut self,
        name: String,
        comment: Option<String>,
        currency: String,
        timezone: String,
        country: Option<String>,
        visible: bool,
    ) -> Result<(), crate::Error> {
        self.record_that(
            CustomerEvent::Updated {
                name,
                comment,
                currency,
                timezone,
                country,
                visible,
            }
            .into(),
        )
        .map_err(|e| customer::DomainError::AggregateError(e).into())
    }

    /// # Errors
    ///
    /// Returns an error if the domain event cannot be applied to the aggregate.
    pub fn set_budget(
        &mut self,
        time_budget: Option<i32>,
        money_budget: Option<i64>,
        budget_is_monthly: bool,
    ) -> Result<(), crate::Error> {
        self.record_that(
            CustomerEvent::BudgetUpdated {
                time_budget,
                money_budget,
                budget_is_monthly,
            }
            .into(),
        )
        .map_err(|e| customer::DomainError::AggregateError(e).into())
    }
}
