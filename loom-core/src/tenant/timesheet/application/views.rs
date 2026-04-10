use crate::shared::AggregateId;
use crate::tenant::activity::ActivityId;
use crate::tenant::project::ProjectId;
use crate::tenant::timesheet::TimesheetId;

pub type UserId = AggregateId;

#[derive(Debug, Clone)]
pub struct TimesheetView {
    id: TimesheetId,
    user_id: UserId,
    project_id: ProjectId,
    activity_id: ActivityId,
    start_time: String,
    end_time: Option<String>,
    duration: Option<i32>,
    description: Option<String>,
    timezone: String,
    billable: bool,
    hourly_rate: Option<i64>,
    fixed_rate: Option<i64>,
    internal_rate: Option<i64>,
    rate: Option<i64>,
    exported: bool,
}

impl TimesheetView {
    #[allow(clippy::too_many_arguments)]
    #[must_use] 
    pub const fn new(
        id: TimesheetId,
        user_id: UserId,
        project_id: ProjectId,
        activity_id: ActivityId,
        start_time: String,
        end_time: Option<String>,
        duration: Option<i32>,
        description: Option<String>,
        timezone: String,
        billable: bool,
        hourly_rate: Option<i64>,
        fixed_rate: Option<i64>,
        internal_rate: Option<i64>,
        rate: Option<i64>,
        exported: bool,
    ) -> Self {
        Self {
            id,
            user_id,
            project_id,
            activity_id,
            start_time,
            end_time,
            duration,
            description,
            timezone,
            billable,
            hourly_rate,
            fixed_rate,
            internal_rate,
            rate,
            exported,
        }
    }

    #[must_use] 
    pub const fn get_id(&self) -> &TimesheetId {
        &self.id
    }
    #[must_use] 
    pub const fn get_user_id(&self) -> &UserId {
        &self.user_id
    }
    #[must_use] 
    pub const fn get_project_id(&self) -> &ProjectId {
        &self.project_id
    }
    #[must_use] 
    pub const fn get_activity_id(&self) -> &ActivityId {
        &self.activity_id
    }
    #[must_use] 
    pub fn get_start_time(&self) -> &str {
        &self.start_time
    }
    #[must_use] 
    pub fn get_end_time(&self) -> Option<&str> {
        self.end_time.as_deref()
    }
    #[must_use] 
    pub const fn get_duration(&self) -> Option<i32> {
        self.duration
    }
    #[must_use] 
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    #[must_use] 
    pub fn get_timezone(&self) -> &str {
        &self.timezone
    }
    #[must_use] 
    pub const fn is_billable(&self) -> bool {
        self.billable
    }
    #[must_use] 
    pub const fn get_hourly_rate(&self) -> Option<i64> {
        self.hourly_rate
    }
    #[must_use] 
    pub const fn get_fixed_rate(&self) -> Option<i64> {
        self.fixed_rate
    }
    #[must_use] 
    pub const fn get_internal_rate(&self) -> Option<i64> {
        self.internal_rate
    }
    #[must_use] 
    pub const fn get_rate(&self) -> Option<i64> {
        self.rate
    }
    #[must_use] 
    pub const fn is_exported(&self) -> bool {
        self.exported
    }
}
