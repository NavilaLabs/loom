use eventually::message::Message;
use serde::{Deserialize, Serialize};

use crate::tenant::tag::TagId;
use crate::tenant::timesheet::TimesheetId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TagEvent {
    Created { id: TagId, name: String },
    Renamed { name: String },
    TimesheetTagged { timesheet_id: TimesheetId },
    TimesheetUntagged { timesheet_id: TimesheetId },
}

impl Message for TagEvent {
    fn name(&self) -> &'static str {
        match self {
            Self::Created { .. } => "TagCreated",
            Self::Renamed { .. } => "TagRenamed",
            Self::TimesheetTagged { .. } => "TagTimesheetTagged",
            Self::TimesheetUntagged { .. } => "TagTimesheetUntagged",
        }
    }
}
