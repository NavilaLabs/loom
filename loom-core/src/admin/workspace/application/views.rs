use crate::admin::workspace::WorkspaceId;

#[derive(Debug, Clone)]
pub struct WorkspaceView {
    id: WorkspaceId,
    name: Option<String>,
}

impl WorkspaceView {
    pub fn new(id: WorkspaceId, name: Option<String>) -> Self {
        Self { id, name }
    }

    pub fn get_id(&self) -> &WorkspaceId {
        &self.id
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}
