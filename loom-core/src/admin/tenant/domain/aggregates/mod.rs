use eventually::aggregate::Aggregate;
use serde::{Deserialize, Serialize};

use crate::{AggregateId, admin::tenant::events::TenantEvent};

pub type TenantId = AggregateId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tenant {
    id: TenantId,
    name: String,
}

impl Tenant {
    pub fn id(&self) -> &TenantId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Tenant already exists")]
    AlreadyExists,
}

impl Aggregate for Tenant {
    type Id = TenantId;
    type Event = TenantEvent;
    type Error = Error;

    fn type_name() -> &'static str {
        "Tenant"
    }

    fn aggregate_id(&self) -> &Self::Id {
        &self.id
    }

    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
        match (state, event) {
            (None, TenantEvent::Created { id, name }) => Ok(Self { id, name }),
            (Some(_), TenantEvent::Created { .. }) => Err(Error::AlreadyExists),
        }
    }
}
