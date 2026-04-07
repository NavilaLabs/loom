use crate::admin::workspace_role::domain::aggregates::WorkspaceRole;
use eventually::aggregate::repository::{Getter, Saver};

pub trait WorkspaceRoleRepository:
    Getter<WorkspaceRole> + Saver<WorkspaceRole> + Send + Sync
{
}
