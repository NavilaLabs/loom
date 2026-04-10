use crate::admin::{workspace::WorkspaceId, workspace_role::WorkspaceRoleId};

#[derive(Debug, Clone)]
pub struct WorkspaceRoleView {
    id: WorkspaceRoleId,
    workspace_id: WorkspaceId,
    name: Option<String>,
}

impl WorkspaceRoleView {
    #[must_use] 
    pub const fn new(id: WorkspaceRoleId, workspace_id: WorkspaceId, name: Option<String>) -> Self {
        Self {
            id,
            workspace_id,
            name,
        }
    }

    #[must_use] 
    pub const fn get_id(&self) -> &WorkspaceRoleId {
        &self.id
    }

    #[must_use] 
    pub const fn get_workspace_id(&self) -> &WorkspaceId {
        &self.workspace_id
    }

    #[must_use] 
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}
