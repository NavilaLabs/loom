use crate::admin::user::UserId;

#[derive(Debug, Clone)]
pub struct UserView {
    id: UserId,
    name: String,
    email: String,
    pub timezone: String,
    pub date_format: String,
    pub language: String,
}

impl UserView {
    pub fn new(id: UserId, name: String, email: String) -> Self {
        Self {
            id,
            name,
            email,
            timezone: "UTC".to_string(),
            date_format: "%Y-%m-%d".to_string(),
            language: "en".to_string(),
        }
    }

    pub fn new_with_settings(
        id: UserId,
        name: String,
        email: String,
        timezone: String,
        date_format: String,
        language: String,
    ) -> Self {
        Self {
            id,
            name,
            email,
            timezone,
            date_format,
            language,
        }
    }

    pub fn get_id(&self) -> &UserId {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }
}
