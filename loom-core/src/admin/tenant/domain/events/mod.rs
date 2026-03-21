use eventually::message::Message;
use serde::{Deserialize, Serialize};

use crate::admin::tenant::aggregates::TenantId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TenantEvent {
    Created { id: TenantId, name: String },
}

impl Message for TenantEvent {
    fn name(&self) -> &'static str {
        match self {
            TenantEvent::Created { .. } => "TenantCreated",
        }
    }
}
