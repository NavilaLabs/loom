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
    tenant::timesheet::repositories::{TimesheetRepository, TimesheetRow},
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
    project_id: String,
    activity_id: String,
    description: Option<String>,
    billable: bool,
) -> Result<TimesheetRow> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = TimesheetRepository::from_pool(pool).await?;

    let id = TimesheetId::new();
    let uid: AggregateId = user_id.parse()?;
    let pid: ProjectId = project_id.parse()?;
    let aid: ActivityId = activity_id.parse()?;
    let start_time = Utc::now().to_rfc3339();
    let timezone = "UTC".to_string();

    let mut root = Root::<Timesheet>::record_new(
        TimesheetEvent::Started {
            id: id.clone(),
            user_id: uid,
            project_id: pid.clone(),
            activity_id: aid.clone(),
            start_time: start_time.clone(),
            timezone: timezone.clone(),
            billable,
        }
        .into(),
    )?;
    // Attach description via an immediate update if provided
    if description.is_some() {
        root.record_that(
            TimesheetEvent::Updated { description: description.clone(), billable }.into(),
        )?;
    }
    repo.save(&mut root).await?;

    Ok(TimesheetRow {
        id: id.to_string(),
        user_id: user_id.to_string(),
        project_id: pid.to_string(),
        activity_id: aid.to_string(),
        start_time,
        end_time: None,
        duration: None,
        description,
        timezone,
        billable,
        exported: false,
    })
}

pub async fn stop(workspace_id: &str, timesheet_id: &str) -> Result<()> {
    let pool = tenant_pool(workspace_id).await?;
    let repo = TimesheetRepository::from_pool(pool).await?;

    let agg_id: TimesheetId = timesheet_id.parse()?;
    let mut root = repo.get(&agg_id).await?;

    // Retrieve the start_time from the aggregate state to compute duration
    let end_time = Utc::now();
    let end_rfc = end_time.to_rfc3339();

    // Parse start_time from the aggregate; fall back to 0 duration on error.
    let duration = chrono::DateTime::parse_from_rfc3339(root.start_time())
        .ok()
        .map(|start| (end_time - start.with_timezone(&Utc)).num_seconds() as i32)
        .unwrap_or(0);

    root.record_that(
        TimesheetEvent::Stopped {
            end_time: end_rfc,
            duration,
            hourly_rate: None,
            fixed_rate: None,
            internal_rate: None,
            rate: None,
        }
        .into(),
    )?;
    repo.save(&mut root).await?;
    Ok(())
}
