use crate::admin::{workspace::WorkspaceId, workspace_role::WorkspaceRoleId};

#[derive(Debug, Clone)]
pub struct WorkspaceRoleView {
    id: WorkspaceRoleId,
    workspace_id: WorkspaceId,
    name: Option<String>,
}

impl WorkspaceRoleView {
    pub fn new(id: WorkspaceRoleId, workspace_id: WorkspaceId, name: Option<String>) -> Self {
        Self {
            id,
            workspace_id,
            name,
        }
    }

    pub fn get_id(&self) -> &WorkspaceRoleId {
        &self.id
    }

    pub fn get_workspace_id(&self) -> &WorkspaceId {
        &self.workspace_id
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}
