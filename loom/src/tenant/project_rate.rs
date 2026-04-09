use anyhow::Result;
use eventually::aggregate::{Root, repository::{Getter, Saver}};
use loom_core::tenant::{
    project::ProjectId,
    project_rate::{ProjectRate, ProjectRateEvent, ProjectRateId},
};
use loom_infrastructure_impl::{
    Pool, ScopeTenant, StateDisconnected,
    tenant::project_rate::repositories::{ProjectRateRepository, ProjectRateRow},
};

async fn tenant_pool(workspace_id: &str) -> Result<loom_infrastructure_impl::ConnectedTenantPool> {
    Ok(Pool::<ScopeTenant, StateDisconnected>::connect_tenant(workspace_id).await?)
}

pub async fn list_for_project(workspace_id: &str, project_id: &str) -> Result<Vec<ProjectRateRow>> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = ProjectRateRepository::from_pool(pool).await?;
    Ok(repo.for_project(project_id).await?)
}

pub async fn set_default(
    workspace_id: &str,
    project_id: String,
    hourly_rate: i64,
    internal_rate: Option<i64>,
) -> Result<ProjectRateRow> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = ProjectRateRepository::from_pool(pool).await?;

    // Remove existing default rate (user_id IS NULL) for this project if any
    if let Some(existing) = repo.default_for_project(&project_id).await? {
        let existing_id: ProjectRateId = existing.id.parse()?;
        let mut root = repo.get(&existing_id).await?;
        root.record_that(ProjectRateEvent::Removed.into())?;
        repo.save(&mut root).await?;
    }

    let id = ProjectRateId::new();
    let pid: ProjectId = project_id.parse()?;
    let mut root = Root::<ProjectRate>::record_new(
        ProjectRateEvent::Set {
            id: id.clone(),
            project_id: pid,
            user_id: None,
            hourly_rate,
            internal_rate,
        }
        .into(),
    )?;
    repo.save(&mut root).await?;

    Ok(ProjectRateRow {
        id: id.to_string(),
        project_id,
        user_id: None,
        hourly_rate,
        internal_rate,
    })
}

pub async fn remove_default(workspace_id: &str, project_id: &str) -> Result<()> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = ProjectRateRepository::from_pool(pool).await?;
    if let Some(existing) = repo.default_for_project(project_id).await? {
        let existing_id: ProjectRateId = existing.id.parse()?;
        let mut root = repo.get(&existing_id).await?;
        root.record_that(ProjectRateEvent::Removed.into())?;
        repo.save(&mut root).await?;
    }
    Ok(())
}
