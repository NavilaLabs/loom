use crate::admin::user::UserId;

#[derive(Debug, Clone)]
pub struct UserView {
    id: UserId,
    name: String,
}

impl UserView {
    pub fn new(id: UserId, name: String) -> Self {
        Self { id, name }
    }

    pub fn get_id(&self) -> &UserId {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
