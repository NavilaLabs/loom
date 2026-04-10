use eventually::aggregate::Aggregate;
use serde::{Deserialize, Serialize};

use crate::shared::AggregateId;
use crate::tenant::tag::TagEvent;

pub type TagId = AggregateId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    id: TagId,
    name: String,
}

impl Tag {
    pub fn id(&self) -> &TagId {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

crate::aggregate_errors!("tag");

impl Aggregate for Tag {
    type Id = TagId;
    type Event = TagEvent;
    type Error = Error;

    fn type_name() -> &'static str {
        "tag"
    }

    fn aggregate_id(&self) -> &Self::Id {
        &self.id
    }

    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
        match (state, event) {
            (None, TagEvent::Created { id, name }) => Ok(Self { id, name }),
            (Some(_), TagEvent::Created { .. }) => Err(Error::AlreadyExists),
            (None, _) => Err(Error::NotFound),
            (Some(mut t), TagEvent::Renamed { name }) => {
                t.name = name;
                Ok(t)
            }
            (Some(t), TagEvent::TimesheetTagged { .. } | TagEvent::TimesheetUntagged { .. }) => {
                Ok(t)
            }
        }
    }
}
