use anyhow::Result;
use eventually::aggregate::{Root, repository::{Getter, Saver}};
use loom_core::tenant::{customer::CustomerId, project::{Project, ProjectEvent, ProjectId}};
use loom_infrastructure_impl::{
    Pool, ScopeTenant, StateDisconnected,
    tenant::project::repositories::{ProjectRepository, ProjectRow},
};

async fn tenant_pool(workspace_id: &str) -> Result<loom_infrastructure_impl::ConnectedTenantPool> {
    Ok(Pool::<ScopeTenant, StateDisconnected>::connect_tenant(workspace_id).await?)
}

pub async fn list(workspace_id: &str) -> Result<Vec<ProjectRow>> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = ProjectRepository::from_pool(pool).await?;
    Ok(repo.all().await?)
}

pub async fn create(
    workspace_id: &str,
    customer_id: String,
    name: String,
) -> Result<ProjectRow> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = ProjectRepository::from_pool(pool).await?;
    let id = ProjectId::new();
    let cid: CustomerId = customer_id.parse()?;
    let mut root = Root::<Project>::record_new(
        ProjectEvent::Created { id: id.clone(), customer_id: cid.clone(), name: name.clone() }
            .into(),
    )?;
    repo.save(&mut root).await?;
    Ok(ProjectRow {
        id: id.to_string(),
        customer_id: cid.to_string(),
        name,
        comment: None,
        order_number: None,
        visible: true,
        billable: true,
        time_budget: None,
        money_budget: None,
        budget_is_monthly: false,
    })
}

pub async fn update(
    workspace_id: &str,
    id: &str,
    name: String,
    comment: Option<String>,
    order_number: Option<String>,
    visible: bool,
    billable: bool,
) -> Result<()> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = ProjectRepository::from_pool(pool).await?;
    let agg_id: ProjectId = id.parse()?;
    let mut root = repo.get(&agg_id).await?;
    root.record_that(
        ProjectEvent::Updated { name, comment, order_number, visible, billable }.into(),
    )?;
    repo.save(&mut root).await?;
    Ok(())
}

pub async fn set_budget(
    workspace_id: &str,
    id: &str,
    time_budget: Option<i32>,
    money_budget: Option<i64>,
    budget_is_monthly: bool,
) -> Result<()> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = ProjectRepository::from_pool(pool).await?;
    let agg_id: ProjectId = id.parse()?;
    let mut root = repo.get(&agg_id).await?;
    root.record_that(
        ProjectEvent::BudgetUpdated { time_budget, money_budget, budget_is_monthly }.into(),
    )?;
    repo.save(&mut root).await?;
    Ok(())
}
