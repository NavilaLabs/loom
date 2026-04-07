use eventually::message::Message;
use serde::{Deserialize, Serialize};

use crate::admin::{
    permission::PermissionId,
    workspace::WorkspaceId,
    workspace_role::WorkspaceRoleId,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceRoleEvent {
    Created {
        id: WorkspaceRoleId,
        workspace_id: WorkspaceId,
        name: Option<String>,
    },
    PermissionGranted {
        permission_id: PermissionId,
    },
    PermissionRevoked {
        permission_id: PermissionId,
    },
}

impl Message for WorkspaceRoleEvent {
    fn name(&self) -> &'static str {
        match self {
            WorkspaceRoleEvent::Created { .. } => "WorkspaceRoleCreated",
            WorkspaceRoleEvent::PermissionGranted { .. } => "WorkspaceRolePermissionGranted",
            WorkspaceRoleEvent::PermissionRevoked { .. } => "WorkspaceRolePermissionRevoked",
        }
    }
}
