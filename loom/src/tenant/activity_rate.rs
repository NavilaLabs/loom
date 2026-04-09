use anyhow::Result;
use eventually::aggregate::{Root, repository::{Getter, Saver}};
use loom_core::tenant::{
    activity::ActivityId,
    activity_rate::{ActivityRate, ActivityRateEvent, ActivityRateId},
};
use loom_infrastructure_impl::{
    Pool, ScopeTenant, StateDisconnected,
    tenant::activity_rate::repositories::{ActivityRateRepository, ActivityRateRow},
};

async fn tenant_pool(workspace_id: &str) -> Result<loom_infrastructure_impl::ConnectedTenantPool> {
    Ok(Pool::<ScopeTenant, StateDisconnected>::connect_tenant(workspace_id).await?)
}

pub async fn list_for_activity(
    workspace_id: &str,
    activity_id: &str,
) -> Result<Vec<ActivityRateRow>> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = ActivityRateRepository::from_pool(pool).await?;
    Ok(repo.for_activity(activity_id).await?)
}

pub async fn set_default(
    workspace_id: &str,
    activity_id: String,
    hourly_rate: i64,
    internal_rate: Option<i64>,
) -> Result<ActivityRateRow> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = ActivityRateRepository::from_pool(pool).await?;

    // Remove existing default rate (user_id IS NULL) for this activity if any
    if let Some(existing) = repo.default_for_activity(&activity_id).await? {
        let existing_id: ActivityRateId = existing.id.parse()?;
        let mut root = repo.get(&existing_id).await?;
        root.record_that(ActivityRateEvent::Removed.into())?;
        repo.save(&mut root).await?;
    }

    let id = ActivityRateId::new();
    let aid: ActivityId = activity_id.parse()?;
    let mut root = Root::<ActivityRate>::record_new(
        ActivityRateEvent::Set {
            id: id.clone(),
            activity_id: aid,
            user_id: None,
            hourly_rate,
            internal_rate,
        }
        .into(),
    )?;
    repo.save(&mut root).await?;

    Ok(ActivityRateRow {
        id: id.to_string(),
        activity_id,
        user_id: None,
        hourly_rate,
        internal_rate,
    })
}

pub async fn remove_default(workspace_id: &str, activity_id: &str) -> Result<()> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = ActivityRateRepository::from_pool(pool).await?;
    if let Some(existing) = repo.default_for_activity(activity_id).await? {
        let existing_id: ActivityRateId = existing.id.parse()?;
        let mut root = repo.get(&existing_id).await?;
        root.record_that(ActivityRateEvent::Removed.into())?;
        repo.save(&mut root).await?;
    }
    Ok(())
}
