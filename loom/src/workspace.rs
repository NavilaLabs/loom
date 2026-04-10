use anyhow::Result;
use loom_infrastructure_impl::{Pool, admin::workspace::repositories::WorkspaceRepository};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    pub id: String,
    pub name: Option<String>,
}

/// Returns all workspaces the given user is a member of.
pub async fn list_user_workspaces(user_id: &str) -> Result<Vec<WorkspaceInfo>> {
    let pool = Pool::connect_admin().await?;
    let repo = WorkspaceRepository::from_pool(pool).await?;
    let rows = repo.find_workspaces_for_user(user_id).await?;
    Ok(rows
        .into_iter()
        .map(|(id, name)| WorkspaceInfo { id, name })
        .collect())
}
