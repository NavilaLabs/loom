use crate::admin::permission::PermissionId;

#[derive(Debug, Clone)]
pub struct PermissionView {
    id: PermissionId,
    name: String,
}

impl PermissionView {
    #[must_use]
    pub const fn new(id: PermissionId, name: String) -> Self {
        Self { id, name }
    }

    #[must_use]
    pub const fn get_id(&self) -> &PermissionId {
        &self.id
    }

    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
