use eventually::message::Message;
use serde::{Deserialize, Serialize};

use crate::shared::AggregateId;
use crate::tenant::activity::ActivityId;
use crate::tenant::activity_rate::ActivityRateId;

/// User ID is an admin-domain user — stored as a plain AggregateId.
pub type UserId = AggregateId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActivityRateEvent {
    Set {
        id: ActivityRateId,
        activity_id: ActivityId,
        /// `None` means the rate applies to all users of this activity.
        user_id: Option<UserId>,
        hourly_rate: i64,
        internal_rate: Option<i64>,
    },
    Removed,
}

impl Message for ActivityRateEvent {
    fn name(&self) -> &'static str {
        match self {
            ActivityRateEvent::Set { .. } => "ActivityRateSet",
            ActivityRateEvent::Removed => "ActivityRateRemoved",
        }
    }
}
