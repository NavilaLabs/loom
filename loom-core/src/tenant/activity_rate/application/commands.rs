use eventually::aggregate;

use crate::tenant::activity::ActivityId;
use crate::tenant::activity_rate::{
    self,
    domain::{
        aggregates::{ActivityRate, ActivityRateId},
        events::{ActivityRateEvent, UserId},
    },
};

#[eventually_macros::aggregate_root(ActivityRate)]
pub struct ActivityRateCommand;

impl ActivityRateCommand {
    pub fn set(
        &self,
        id: ActivityRateId,
        activity_id: ActivityId,
        user_id: Option<UserId>,
        hourly_rate: i64,
        internal_rate: Option<i64>,
    ) -> Result<Self, crate::Error> {
        Ok(
            aggregate::Root::<ActivityRate>::record_new(
                ActivityRateEvent::Set { id, activity_id, user_id, hourly_rate, internal_rate }
                    .into(),
            )
            .map_err(activity_rate::DomainError::from)?
            .into(),
        )
    }

    pub fn remove(&mut self) -> Result<(), crate::Error> {
        self.record_that(ActivityRateEvent::Removed.into())
            .map_err(|e| activity_rate::DomainError::AggregateError(e).into())
    }
}
