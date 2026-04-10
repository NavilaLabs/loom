use eventually::aggregate::Aggregate;
use serde::{Deserialize, Serialize};

use crate::shared::AggregateId;
use crate::tenant::project::ProjectId;
use crate::tenant::project_rate::ProjectRateEvent;
use crate::tenant::project_rate::domain::events::UserId;

pub type ProjectRateId = AggregateId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectRate {
    id: ProjectRateId,
    project_id: ProjectId,
    user_id: Option<UserId>,
    hourly_rate: i64,
    internal_rate: Option<i64>,
}

impl ProjectRate {
    pub fn id(&self) -> &ProjectRateId {
        &self.id
    }
    pub fn project_id(&self) -> &ProjectId {
        &self.project_id
    }
    pub fn user_id(&self) -> Option<&UserId> {
        self.user_id.as_ref()
    }
    pub fn hourly_rate(&self) -> i64 {
        self.hourly_rate
    }
    pub fn internal_rate(&self) -> Option<i64> {
        self.internal_rate
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("project rate already exists")]
    AlreadyExists,
    #[error("project rate not found")]
    NotFound,
}

impl Aggregate for ProjectRate {
    type Id = ProjectRateId;
    type Event = ProjectRateEvent;
    type Error = Error;

    fn type_name() -> &'static str {
        "project_rate"
    }

    fn aggregate_id(&self) -> &Self::Id {
        &self.id
    }

    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
        match (state, event) {
            (
                None,
                ProjectRateEvent::Set {
                    id,
                    project_id,
                    user_id,
                    hourly_rate,
                    internal_rate,
                },
            ) => Ok(Self {
                id,
                project_id,
                user_id,
                hourly_rate,
                internal_rate,
            }),
            (Some(_), ProjectRateEvent::Set { .. }) => Err(Error::AlreadyExists),
            (None, _) => Err(Error::NotFound),
            // Removed is a terminal event — the aggregate is gone from the store.
            (Some(r), ProjectRateEvent::Removed) => Ok(r),
        }
    }
}
