use crate::tenant::tag::TagId;

#[derive(Debug, Clone)]
pub struct TagView {
    id: TagId,
    name: String,
}

impl TagView {
    #[must_use]
    pub const fn new(id: TagId, name: String) -> Self {
        Self { id, name }
    }

    #[must_use]
    pub const fn get_id(&self) -> &TagId {
        &self.id
    }
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
