use crate::shared::AggregateId;
use crate::tenant::project::ProjectId;
use crate::tenant::project_rate::ProjectRateId;

pub type UserId = AggregateId;

#[derive(Debug, Clone)]
pub struct ProjectRateView {
    id: ProjectRateId,
    project_id: ProjectId,
    user_id: Option<UserId>,
    hourly_rate: i64,
    internal_rate: Option<i64>,
}

impl ProjectRateView {
    #[must_use] 
    pub const fn new(
        id: ProjectRateId,
        project_id: ProjectId,
        user_id: Option<UserId>,
        hourly_rate: i64,
        internal_rate: Option<i64>,
    ) -> Self {
        Self {
            id,
            project_id,
            user_id,
            hourly_rate,
            internal_rate,
        }
    }

    #[must_use] 
    pub const fn get_id(&self) -> &ProjectRateId {
        &self.id
    }
    #[must_use] 
    pub const fn get_project_id(&self) -> &ProjectId {
        &self.project_id
    }
    #[must_use] 
    pub const fn get_user_id(&self) -> Option<&UserId> {
        self.user_id.as_ref()
    }
    #[must_use] 
    pub const fn get_hourly_rate(&self) -> i64 {
        self.hourly_rate
    }
    #[must_use] 
    pub const fn get_internal_rate(&self) -> Option<i64> {
        self.internal_rate
    }
}
