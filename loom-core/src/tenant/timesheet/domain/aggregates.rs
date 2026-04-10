use eventually::aggregate::Aggregate;
use serde::{Deserialize, Serialize};

use crate::shared::AggregateId;
use crate::tenant::activity::ActivityId;
use crate::tenant::project::ProjectId;
use crate::tenant::timesheet::TimesheetEvent;
use crate::tenant::timesheet::domain::events::UserId;

pub type TimesheetId = AggregateId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timesheet {
    id: TimesheetId,
    user_id: UserId,
    project_id: Option<ProjectId>,
    activity_id: Option<ActivityId>,
    start_time: String,
    end_time: Option<String>,
    duration: Option<i32>,
    description: Option<String>,
    timezone: String,
    billable: bool,
    exported: bool,
}

impl Timesheet {
    pub fn id(&self) -> &TimesheetId {
        &self.id
    }
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
    pub fn project_id(&self) -> Option<&ProjectId> {
        self.project_id.as_ref()
    }
    pub fn activity_id(&self) -> Option<&ActivityId> {
        self.activity_id.as_ref()
    }
    pub fn start_time(&self) -> &str {
        &self.start_time
    }
    pub fn end_time(&self) -> Option<&str> {
        self.end_time.as_deref()
    }
    pub fn duration(&self) -> Option<i32> {
        self.duration
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn timezone(&self) -> &str {
        &self.timezone
    }
    pub fn billable(&self) -> bool {
        self.billable
    }
    pub fn exported(&self) -> bool {
        self.exported
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("timesheet already exists")]
    AlreadyExists,
    #[error("timesheet not found")]
    NotFound,
    #[error("timesheet already exported")]
    AlreadyExported,
}

impl Aggregate for Timesheet {
    type Id = TimesheetId;
    type Event = TimesheetEvent;
    type Error = Error;

    fn type_name() -> &'static str {
        "timesheet"
    }

    fn aggregate_id(&self) -> &Self::Id {
        &self.id
    }

    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
        match (state, event) {
            (
                None,
                TimesheetEvent::Started {
                    id,
                    user_id,
                    project_id,
                    activity_id,
                    start_time,
                    timezone,
                    billable,
                },
            ) => Ok(Self {
                id,
                user_id,
                project_id,
                activity_id,
                start_time,
                end_time: None,
                duration: None,
                description: None,
                timezone,
                billable,
                exported: false,
            }),
            (Some(_), TimesheetEvent::Started { .. }) => Err(Error::AlreadyExists),
            (None, _) => Err(Error::NotFound),
            (
                Some(mut t),
                TimesheetEvent::Stopped {
                    end_time, duration, ..
                },
            ) => {
                t.end_time = Some(end_time);
                t.duration = Some(duration);
                Ok(t)
            }
            (
                Some(mut t),
                TimesheetEvent::Updated {
                    description,
                    billable,
                },
            ) => {
                t.description = description;
                t.billable = billable;
                Ok(t)
            }
            (
                Some(mut t),
                TimesheetEvent::Reassigned {
                    project_id,
                    activity_id,
                },
            ) => {
                t.project_id = Some(project_id);
                t.activity_id = Some(activity_id);
                Ok(t)
            }
            (
                Some(mut t),
                TimesheetEvent::TimeUpdated {
                    start_time,
                    end_time,
                    duration,
                },
            ) => {
                t.start_time = start_time;
                t.end_time = end_time;
                t.duration = duration;
                Ok(t)
            }
            (Some(t), TimesheetEvent::Exported) => {
                if t.exported {
                    return Err(Error::AlreadyExported);
                }
                Ok(Self {
                    exported: true,
                    ..t
                })
            }
        }
    }
}
