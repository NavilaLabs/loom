use crate::tenant::activity::ActivityId;
use crate::tenant::project::ProjectId;

#[derive(Debug, Clone)]
pub struct ActivityView {
    id: ActivityId,
    project_id: Option<ProjectId>,
    name: String,
    comment: Option<String>,
    visible: bool,
    billable: bool,
}

impl ActivityView {
    #[must_use]
    pub const fn new(
        id: ActivityId,
        project_id: Option<ProjectId>,
        name: String,
        comment: Option<String>,
        visible: bool,
        billable: bool,
    ) -> Self {
        Self {
            id,
            project_id,
            name,
            comment,
            visible,
            billable,
        }
    }

    #[must_use]
    pub const fn get_id(&self) -> &ActivityId {
        &self.id
    }
    #[must_use]
    pub const fn get_project_id(&self) -> Option<&ProjectId> {
        self.project_id.as_ref()
    }
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }
    #[must_use]
    pub fn get_comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }
    #[must_use]
    pub const fn is_visible(&self) -> bool {
        self.visible
    }
    #[must_use]
    pub const fn is_billable(&self) -> bool {
        self.billable
    }
}
