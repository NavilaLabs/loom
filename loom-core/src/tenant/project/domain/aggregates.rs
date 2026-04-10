use eventually::aggregate::Aggregate;
use serde::{Deserialize, Serialize};

use crate::shared::AggregateId;
use crate::tenant::customer::CustomerId;
use crate::tenant::project::ProjectEvent;

pub type ProjectId = AggregateId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    id: ProjectId,
    customer_id: CustomerId,
    name: String,
    visible: bool,
    billable: bool,
}

impl Project {
    pub fn id(&self) -> &ProjectId {
        &self.id
    }
    pub fn customer_id(&self) -> &CustomerId {
        &self.customer_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn visible(&self) -> bool {
        self.visible
    }
    pub fn billable(&self) -> bool {
        self.billable
    }
}

crate::aggregate_errors!("project");

impl Aggregate for Project {
    type Id = ProjectId;
    type Event = ProjectEvent;
    type Error = Error;

    fn type_name() -> &'static str {
        "project"
    }

    fn aggregate_id(&self) -> &Self::Id {
        &self.id
    }

    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
        match (state, event) {
            (
                None,
                ProjectEvent::Created {
                    id,
                    customer_id,
                    name,
                },
            ) => Ok(Self {
                id,
                customer_id,
                name,
                visible: true,
                billable: true,
            }),
            (Some(_), ProjectEvent::Created { .. }) => Err(Error::AlreadyExists),
            (None, _) => Err(Error::NotFound),
            (
                Some(mut p),
                ProjectEvent::Updated {
                    name,
                    visible,
                    billable,
                    ..
                },
            ) => {
                p.name = name;
                p.visible = visible;
                p.billable = billable;
                Ok(p)
            }
            (Some(p), ProjectEvent::BudgetUpdated { .. }) => Ok(p),
        }
    }
}
