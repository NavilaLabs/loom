use eventually::aggregate::Aggregate;
use serde::{Deserialize, Serialize};

use crate::shared::AggregateId;
use crate::tenant::activity::ActivityEvent;
use crate::tenant::project::ProjectId;

pub type ActivityId = AggregateId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Activity {
    id: ActivityId,
    project_id: Option<ProjectId>,
    name: String,
    visible: bool,
    billable: bool,
}

impl Activity {
    #[must_use]
    pub const fn id(&self) -> &ActivityId {
        &self.id
    }
    #[must_use]
    pub const fn project_id(&self) -> Option<&ProjectId> {
        self.project_id.as_ref()
    }
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[must_use]
    pub const fn visible(&self) -> bool {
        self.visible
    }
    #[must_use]
    pub const fn billable(&self) -> bool {
        self.billable
    }
}

crate::aggregate_errors!("activity");

impl Aggregate for Activity {
    type Id = ActivityId;
    type Event = ActivityEvent;
    type Error = Error;

    fn type_name() -> &'static str {
        "activity"
    }

    fn aggregate_id(&self) -> &Self::Id {
        &self.id
    }

    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
        match (state, event) {
            (
                None,
                ActivityEvent::Created {
                    id,
                    project_id,
                    name,
                },
            ) => Ok(Self {
                id,
                project_id,
                name,
                visible: true,
                billable: true,
            }),
            (Some(_), ActivityEvent::Created { .. }) => Err(Error::AlreadyExists),
            (None, _) => Err(Error::NotFound),
            (
                Some(mut a),
                ActivityEvent::Updated {
                    name,
                    visible,
                    billable,
                    ..
                },
            ) => {
                a.name = name;
                a.visible = visible;
                a.billable = billable;
                Ok(a)
            }
        }
    }
}
