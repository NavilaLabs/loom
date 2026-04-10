use anyhow::Result;
use eventually::aggregate::{
    Root,
    repository::{Getter, Saver},
};
use loom_core::{
    tenant::{
        activity::{Activity, ActivityEvent, ActivityId, CreateActivityInput, UpdateActivityInput},
        project::ProjectId,
    },
    validation::{Validate, validation_summary},
};
use loom_infrastructure_impl::{
    Pool, ScopeTenant, StateDisconnected,
    tenant::activity::repositories::{ActivityRepository, ActivityRow},
};

async fn tenant_pool(workspace_id: &str) -> Result<loom_infrastructure_impl::ConnectedTenantPool> {
    Ok(Pool::<ScopeTenant, StateDisconnected>::connect_tenant(workspace_id).await?)
}

pub async fn list(workspace_id: &str) -> Result<Vec<ActivityRow>> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = ActivityRepository::from_pool(pool).await?;
    Ok(repo.all().await?)
}

pub async fn create(
    workspace_id: &str,
    project_id: Option<String>,
    name: String,
) -> Result<ActivityRow> {
    CreateActivityInput { name: name.clone() }
        .validate()
        .map_err(|e| crate::error::ValidationError::new(validation_summary(&e)))?;

    let pool = tenant_pool(workspace_id).await?;
    let repo = ActivityRepository::from_pool(pool).await?;
    let id = ActivityId::new();
    let pid: Option<ProjectId> = project_id.as_deref().map(|s| s.parse()).transpose()?;
    let mut root = Root::<Activity>::record_new(
        ActivityEvent::Created {
            id: id.clone(),
            project_id: pid.clone(),
            name: name.clone(),
        }
        .into(),
    )?;
    repo.save(&mut root).await?;
    Ok(ActivityRow {
        id: id.to_string(),
        project_id: pid.map(|p| p.to_string()),
        name,
        comment: None,
        visible: true,
        billable: true,
    })
}

pub async fn update(
    workspace_id: &str,
    id: &str,
    name: String,
    comment: Option<String>,
    visible: bool,
    billable: bool,
) -> Result<()> {
    UpdateActivityInput { name: name.clone() }
        .validate()
        .map_err(|e| crate::error::ValidationError::new(validation_summary(&e)))?;

    let pool = tenant_pool(workspace_id).await?;
    let repo = ActivityRepository::from_pool(pool).await?;
    let agg_id: ActivityId = id.parse()?;
    let mut root = repo.get(&agg_id).await?;
    root.record_that(
        ActivityEvent::Updated {
            name,
            comment,
            visible,
            billable,
        }
        .into(),
    )?;
    repo.save(&mut root).await?;
    Ok(())
}
