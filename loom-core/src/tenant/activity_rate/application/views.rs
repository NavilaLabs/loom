use crate::shared::AggregateId;
use crate::tenant::activity::ActivityId;
use crate::tenant::activity_rate::ActivityRateId;

pub type UserId = AggregateId;

#[derive(Debug, Clone)]
pub struct ActivityRateView {
    id: ActivityRateId,
    activity_id: ActivityId,
    user_id: Option<UserId>,
    hourly_rate: i64,
    internal_rate: Option<i64>,
}

impl ActivityRateView {
    pub fn new(
        id: ActivityRateId,
        activity_id: ActivityId,
        user_id: Option<UserId>,
        hourly_rate: i64,
        internal_rate: Option<i64>,
    ) -> Self {
        Self {
            id,
            activity_id,
            user_id,
            hourly_rate,
            internal_rate,
        }
    }

    pub fn get_id(&self) -> &ActivityRateId {
        &self.id
    }
    pub fn get_activity_id(&self) -> &ActivityId {
        &self.activity_id
    }
    pub fn get_user_id(&self) -> Option<&UserId> {
        self.user_id.as_ref()
    }
    pub fn get_hourly_rate(&self) -> i64 {
        self.hourly_rate
    }
    pub fn get_internal_rate(&self) -> Option<i64> {
        self.internal_rate
    }
}
