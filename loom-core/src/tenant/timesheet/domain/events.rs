use eventually::message::Message;
use serde::{Deserialize, Serialize};

use crate::shared::AggregateId;
use crate::tenant::activity::ActivityId;
use crate::tenant::project::ProjectId;
use crate::tenant::timesheet::TimesheetId;

/// User ID references an admin-domain user — stored as a plain AggregateId.
pub type UserId = AggregateId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimesheetEvent {
    Started {
        id: TimesheetId,
        user_id: UserId,
        /// None when started via quick timer — assigned later via `Reassigned`.
        project_id: Option<ProjectId>,
        /// None when started via quick timer — assigned later via `Reassigned`.
        activity_id: Option<ActivityId>,
        /// RFC-3339 timestamp string.
        start_time: String,
        timezone: String,
        billable: bool,
    },
    Stopped {
        /// RFC-3339 timestamp string.
        end_time: String,
        /// Duration in seconds.
        duration: i32,
        hourly_rate: Option<i64>,
        fixed_rate: Option<i64>,
        internal_rate: Option<i64>,
        /// Total amount in cents.
        rate: Option<i64>,
    },
    Updated {
        description: Option<String>,
        billable: bool,
    },
    Reassigned {
        project_id: ProjectId,
        activity_id: ActivityId,
    },
    Exported,
}

impl Message for TimesheetEvent {
    fn name(&self) -> &'static str {
        match self {
            TimesheetEvent::Started { .. } => "TimesheetStarted",
            TimesheetEvent::Stopped { .. } => "TimesheetStopped",
            TimesheetEvent::Updated { .. } => "TimesheetUpdated",
            TimesheetEvent::Reassigned { .. } => "TimesheetReassigned",
            TimesheetEvent::Exported => "TimesheetExported",
        }
    }
}
