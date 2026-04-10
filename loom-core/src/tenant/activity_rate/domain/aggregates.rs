use eventually::aggregate::Aggregate;
use serde::{Deserialize, Serialize};

use crate::shared::AggregateId;
use crate::tenant::activity::ActivityId;
use crate::tenant::activity_rate::ActivityRateEvent;
use crate::tenant::activity_rate::domain::events::UserId;

pub type ActivityRateId = AggregateId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRate {
    id: ActivityRateId,
    activity_id: ActivityId,
    user_id: Option<UserId>,
    hourly_rate: i64,
    internal_rate: Option<i64>,
}

impl ActivityRate {
    #[must_use] 
    pub const fn id(&self) -> &ActivityRateId {
        &self.id
    }
    #[must_use] 
    pub const fn activity_id(&self) -> &ActivityId {
        &self.activity_id
    }
    #[must_use] 
    pub const fn user_id(&self) -> Option<&UserId> {
        self.user_id.as_ref()
    }
    #[must_use] 
    pub const fn hourly_rate(&self) -> i64 {
        self.hourly_rate
    }
    #[must_use] 
    pub const fn internal_rate(&self) -> Option<i64> {
        self.internal_rate
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("activity rate already exists")]
    AlreadyExists,
    #[error("activity rate not found")]
    NotFound,
}

impl Aggregate for ActivityRate {
    type Id = ActivityRateId;
    type Event = ActivityRateEvent;
    type Error = Error;

    fn type_name() -> &'static str {
        "activity_rate"
    }

    fn aggregate_id(&self) -> &Self::Id {
        &self.id
    }

    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
        match (state, event) {
            (
                None,
                ActivityRateEvent::Set {
                    id,
                    activity_id,
                    user_id,
                    hourly_rate,
                    internal_rate,
                },
            ) => Ok(Self {
                id,
                activity_id,
                user_id,
                hourly_rate,
                internal_rate,
            }),
            (Some(_), ActivityRateEvent::Set { .. }) => Err(Error::AlreadyExists),
            (None, _) => Err(Error::NotFound),
            (Some(r), ActivityRateEvent::Removed) => Ok(r),
        }
    }
}
