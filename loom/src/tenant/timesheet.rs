use anyhow::Result;
use chrono::Utc;
use eventually::aggregate::{Root, repository::{Getter, Saver}};
use loom_core::{
    shared::AggregateId,
    tenant::{
        activity::ActivityId,
        project::ProjectId,
        timesheet::{Timesheet, TimesheetEvent, TimesheetId},
    },
};
use loom_infrastructure_impl::{
    Pool, ScopeTenant, StateDisconnected,
    tenant::{
        activity_rate::repositories::ActivityRateRepository,
        project_rate::repositories::ProjectRateRepository,
        timesheet::repositories::{TimesheetRepository, TimesheetRow},
    },
};

async fn tenant_pool(workspace_id: &str) -> Result<loom_infrastructure_impl::ConnectedTenantPool> {
    Ok(Pool::<ScopeTenant, StateDisconnected>::connect_tenant(workspace_id).await?)
}

pub async fn recent(workspace_id: &str, user_id: &str) -> Result<Vec<TimesheetRow>> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = TimesheetRepository::from_pool(pool).await?;
    Ok(repo.recent_for_user(user_id).await?)
}

pub async fn running(workspace_id: &str, user_id: &str) -> Result<Option<TimesheetRow>> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = TimesheetRepository::from_pool(pool).await?;
    Ok(repo.running_for_user(user_id).await?)
}

pub async fn start(
    workspace_id: &str,
    user_id: &str,
    project_id: Option<String>,
    activity_id: Option<String>,
    description: Option<String>,
    billable: bool,
) -> Result<TimesheetRow> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = TimesheetRepository::from_pool(pool).await?;

    let id = TimesheetId::new();
    let uid: AggregateId = user_id.parse()?;
    let pid: Option<ProjectId> = project_id.as_deref().map(|s| s.parse()).transpose()?;
    let aid: Option<ActivityId> = activity_id.as_deref().map(|s| s.parse()).transpose()?;
    let start_time = Utc::now().to_rfc3339();
    let timezone = "UTC".to_string();

    let mut root = Root::<Timesheet>::record_new(
        TimesheetEvent::Started {
            id: id.clone(),
            user_id: uid,
            project_id: pid,
            activity_id: aid,
            start_time: start_time.clone(),
            timezone: timezone.clone(),
            billable,
        }
        .into(),
    )?;
    if description.is_some() {
        root.record_that(
            TimesheetEvent::Updated { description: description.clone(), billable }.into(),
        )?;
    }
    repo.save(&mut root).await?;

    Ok(TimesheetRow {
        id: id.to_string(),
        user_id: user_id.to_string(),
        project_id,
        activity_id,
        start_time,
        end_time: None,
        duration: None,
        description,
        timezone,
        billable,
        exported: false,
        hourly_rate: None,
        fixed_rate: None,
        internal_rate: None,
        rate: None,
    })
}

pub async fn reassign(
    workspace_id: &str,
    timesheet_id: &str,
    project_id: String,
    activity_id: String,
) -> Result<()> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = TimesheetRepository::from_pool(pool).await?;
    let agg_id: TimesheetId = timesheet_id.parse()?;
    let mut root = repo.get(&agg_id).await?;
    let pid: ProjectId = project_id.parse()?;
    let aid: ActivityId = activity_id.parse()?;
    root.record_that(TimesheetEvent::Reassigned { project_id: pid, activity_id: aid }.into())?;
    repo.save(&mut root).await?;
    Ok(())
}

pub async fn update(
    workspace_id: &str,
    timesheet_id: &str,
    description: Option<String>,
    billable: bool,
) -> Result<()> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = TimesheetRepository::from_pool(pool).await?;
    let agg_id: TimesheetId = timesheet_id.parse()?;
    let mut root = repo.get(&agg_id).await?;
    root.record_that(TimesheetEvent::Updated { description, billable }.into())?;
    repo.save(&mut root).await?;
    Ok(())
}

pub async fn stop(workspace_id: &str, timesheet_id: &str) -> Result<()> {
    let pool = tenant_pool(workspace_id).await?;
    let ts_repo = TimesheetRepository::from_pool(pool.clone()).await?;

    let agg_id: TimesheetId = timesheet_id.parse()?;
    let mut root = ts_repo.get(&agg_id).await?;

    let end_time = Utc::now();
    let end_rfc = end_time.to_rfc3339();
    let duration = chrono::DateTime::parse_from_rfc3339(root.start_time())
        .ok()
        .map(|start| (end_time - start.with_timezone(&Utc)).num_seconds() as i32)
        .unwrap_or(0);

    // Look up the applicable rate (only possible when project/activity are assigned)
    let project_id = root.project_id().map(|p| p.to_string());
    let activity_id = root.activity_id().map(|a| a.to_string());
    let (hourly_rate, internal_rate) = match (&project_id, &activity_id) {
        (Some(pid), Some(aid)) => resolve_rate(&pool, pid, aid).await,
        _ => (None, None),
    };
    let rate = hourly_rate.map(|hr| hr * duration as i64 / 3600);

    root.record_that(
        TimesheetEvent::Stopped {
            end_time: end_rfc,
            duration,
            hourly_rate,
            fixed_rate: None,
            internal_rate,
            rate,
        }
        .into(),
    )?;
    ts_repo.save(&mut root).await?;
    Ok(())
}

pub async fn export(workspace_id: &str, timesheet_id: &str) -> Result<()> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = TimesheetRepository::from_pool(pool).await?;
    let agg_id: TimesheetId = timesheet_id.parse()?;
    let mut root = repo.get(&agg_id).await?;
    root.record_that(TimesheetEvent::Exported.into())?;
    repo.save(&mut root).await?;
    Ok(())
}

/// Resolve the billing rate for a timesheet: project rate first, activity rate as fallback.
/// Returns `(hourly_rate_cents, internal_rate_cents)`.
async fn resolve_rate(
    pool: &loom_infrastructure_impl::ConnectedTenantPool,
    project_id: &str,
    activity_id: &str,
) -> (Option<i64>, Option<i64>) {
    if let Ok(repo) = ProjectRateRepository::from_pool(pool.clone()).await {
        if let Ok(Some(row)) = repo.default_for_project(project_id).await {
            return (Some(row.hourly_rate), row.internal_rate);
        }
    }
    if let Ok(repo) = ActivityRateRepository::from_pool(pool.clone()).await {
        if let Ok(Some(row)) = repo.default_for_activity(activity_id).await {
            return (Some(row.hourly_rate), row.internal_rate);
        }
    }
    (None, None)
}
