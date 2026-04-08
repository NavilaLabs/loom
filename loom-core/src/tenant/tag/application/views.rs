use crate::tenant::tag::TagId;

#[derive(Debug, Clone)]
pub struct TagView {
    id: TagId,
    name: String,
}

impl TagView {
    pub fn new(id: TagId, name: String) -> Self {
        Self { id, name }
    }

    pub fn get_id(&self) -> &TagId { &self.id }
    pub fn get_name(&self) -> &str { &self.name }
}
