use eventually::message::Message;
use serde::{Deserialize, Serialize};

use crate::admin::{
    permission::PermissionId, user::UserId, workspace::WorkspaceId, workspace_role::WorkspaceRoleId,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceEvent {
    Created {
        id: WorkspaceId,
        name: Option<String>,
    },
    UserRoleAssigned {
        user_id: UserId,
        workspace_role_id: WorkspaceRoleId,
    },
    UserRoleRevoked {
        user_id: UserId,
        workspace_role_id: WorkspaceRoleId,
    },
    UserPermissionGranted {
        user_id: UserId,
        permission_id: PermissionId,
    },
    UserPermissionRevoked {
        user_id: UserId,
        permission_id: PermissionId,
    },
}

impl Message for WorkspaceEvent {
    fn name(&self) -> &'static str {
        match self {
            WorkspaceEvent::Created { .. } => "WorkspaceCreated",
            WorkspaceEvent::UserRoleAssigned { .. } => "WorkspaceUserRoleAssigned",
            WorkspaceEvent::UserRoleRevoked { .. } => "WorkspaceUserRoleRevoked",
            WorkspaceEvent::UserPermissionGranted { .. } => "WorkspaceUserPermissionGranted",
            WorkspaceEvent::UserPermissionRevoked { .. } => "WorkspaceUserPermissionRevoked",
        }
    }
}
