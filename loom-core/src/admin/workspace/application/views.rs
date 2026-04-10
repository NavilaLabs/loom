use crate::admin::workspace::WorkspaceId;

#[derive(Debug, Clone)]
pub struct WorkspaceView {
    id: WorkspaceId,
    name: Option<String>,
    pub timezone: String,
    pub date_format: String,
    pub currency: String,
    pub week_start: String,
}

impl WorkspaceView {
    pub fn new(id: WorkspaceId, name: Option<String>) -> Self {
        Self {
            id,
            name,
            timezone: "UTC".to_string(),
            date_format: "%Y-%m-%d".to_string(),
            currency: "USD".to_string(),
            week_start: "monday".to_string(),
        }
    }

    pub fn new_with_settings(
        id: WorkspaceId,
        name: Option<String>,
        timezone: String,
        date_format: String,
        currency: String,
        week_start: String,
    ) -> Self {
        Self {
            id,
            name,
            timezone,
            date_format,
            currency,
            week_start,
        }
    }

    pub fn get_id(&self) -> &WorkspaceId {
        &self.id
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}
