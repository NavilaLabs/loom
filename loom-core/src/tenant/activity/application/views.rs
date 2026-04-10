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
    pub fn new(
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

    pub fn get_id(&self) -> &ActivityId {
        &self.id
    }
    pub fn get_project_id(&self) -> Option<&ProjectId> {
        self.project_id.as_ref()
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    pub fn is_billable(&self) -> bool {
        self.billable
    }
}
