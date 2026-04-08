use eventually::message::Message;
use serde::{Deserialize, Serialize};

use crate::shared::AggregateId;
use crate::tenant::project::ProjectId;
use crate::tenant::project_rate::ProjectRateId;

/// User ID is an admin-domain user — stored as a plain AggregateId.
pub type UserId = AggregateId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProjectRateEvent {
    Set {
        id: ProjectRateId,
        project_id: ProjectId,
        /// `None` means the rate applies to all users of this project.
        user_id: Option<UserId>,
        hourly_rate: i64,
        internal_rate: Option<i64>,
    },
    Removed,
}

impl Message for ProjectRateEvent {
    fn name(&self) -> &'static str {
        match self {
            ProjectRateEvent::Set { .. } => "ProjectRateSet",
            ProjectRateEvent::Removed => "ProjectRateRemoved",
        }
    }
}
