use eventually::message::Message;
use serde::{Deserialize, Serialize};

use crate::tenant::activity::ActivityId;
use crate::tenant::project::ProjectId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActivityEvent {
    Created {
        id: ActivityId,
        /// `None` means this is a global (cross-project) activity.
        project_id: Option<ProjectId>,
        name: String,
    },
    Updated {
        name: String,
        comment: Option<String>,
        visible: bool,
        billable: bool,
    },
}

impl Message for ActivityEvent {
    fn name(&self) -> &'static str {
        match self {
            ActivityEvent::Created { .. } => "ActivityCreated",
            ActivityEvent::Updated { .. } => "ActivityUpdated",
        }
    }
}
