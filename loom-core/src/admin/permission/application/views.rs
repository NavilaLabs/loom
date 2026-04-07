use crate::admin::permission::PermissionId;

#[derive(Debug, Clone)]
pub struct PermissionView {
    id: PermissionId,
    name: String,
}

impl PermissionView {
    pub fn new(id: PermissionId, name: String) -> Self {
        Self { id, name }
    }

    pub fn get_id(&self) -> &PermissionId {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
