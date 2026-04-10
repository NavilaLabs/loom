use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimesheetDto {
    pub id: String,
    pub user_id: String,
    pub project_id: Option<String>,
    pub activity_id: Option<String>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration: Option<i32>,
    pub description: Option<String>,
    pub timezone: String,
    pub billable: bool,
    pub exported: bool,
    /// Billable hourly rate snapshot in cents.
    pub hourly_rate: Option<i64>,
    /// Internal (cost) rate snapshot in cents.
    pub internal_rate: Option<i64>,
    /// Total billable amount in cents (`hourly_rate * duration / 3600`).
    pub rate: Option<i64>,
}

#[get("/api/timesheets/recent")]
pub async fn list_timesheets() -> Result<Vec<TimesheetDto>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _list_timesheets().await
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(vec![])
    }
}

#[get("/api/timesheets/running")]
pub async fn running_timesheet() -> Result<Option<TimesheetDto>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _running_timesheet().await
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(None)
    }
}

#[post("/api/timesheets/start")]
pub async fn start_timesheet(
    project_id: Option<String>,
    activity_id: Option<String>,
    description: Option<String>,
    billable: bool,
) -> Result<TimesheetDto, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _start_timesheet(project_id, activity_id, description, billable).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (project_id, activity_id, description, billable);
        Err(ServerFnError::ServerError {
            message: "server only".into(),
            code: 500,
            details: None,
        })
    }
}

#[post("/api/timesheets/reassign")]
pub async fn reassign_timesheet(
    timesheet_id: String,
    project_id: String,
    activity_id: String,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _reassign_timesheet(timesheet_id, project_id, activity_id).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (timesheet_id, project_id, activity_id);
        Ok(())
    }
}

#[post("/api/timesheets/update")]
pub async fn update_timesheet(
    timesheet_id: String,
    description: Option<String>,
    billable: bool,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _update_timesheet(timesheet_id, description, billable).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (timesheet_id, description, billable);
        Ok(())
    }
}

#[post("/api/timesheets/create-manual")]
pub async fn create_timesheet_manual(
    project_id: Option<String>,
    activity_id: Option<String>,
    start_time: String,
    end_time: String,
    description: Option<String>,
    billable: bool,
) -> Result<TimesheetDto, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _create_timesheet_manual(project_id, activity_id, start_time, end_time, description, billable).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (project_id, activity_id, start_time, end_time, description, billable);
        Err(ServerFnError::ServerError {
            message: "server only".into(),
            code: 500,
            details: None,
        })
    }
}

#[post("/api/timesheets/update-time")]
pub async fn update_timesheet_time(
    timesheet_id: String,
    start_time: String,
    end_time: Option<String>,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _update_timesheet_time(timesheet_id, start_time, end_time).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (timesheet_id, start_time, end_time);
        Ok(())
    }
}

#[post("/api/timesheets/stop")]
pub async fn stop_timesheet(timesheet_id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _stop_timesheet(timesheet_id).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = timesheet_id;
        Ok(())
    }
}

#[post("/api/timesheets/export")]
pub async fn export_timesheet(timesheet_id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _export_timesheet(timesheet_id).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = timesheet_id;
        Ok(())
    }
}

#[cfg(feature = "server")]
fn row_to_dto(
    r: loom::infrastructure::tenant::timesheet::repositories::TimesheetRow,
) -> TimesheetDto {
    TimesheetDto {
        id: r.id,
        user_id: r.user_id,
        project_id: r.project_id,
        activity_id: r.activity_id,
        start_time: r.start_time,
        end_time: r.end_time,
        duration: r.duration,
        description: r.description,
        timezone: r.timezone,
        billable: r.billable,
        exported: r.exported,
        hourly_rate: r.hourly_rate,
        internal_rate: r.internal_rate,
        rate: r.rate,
    }
}

#[cfg(feature = "server")]
async fn _list_timesheets() -> Result<Vec<TimesheetDto>, ServerFnError> {
    use crate::session;

    let (user, workspace_id) = session::session_workspace().await?;
    let rows = loom::tenant::timesheet::recent(&workspace_id, &user.id)
        .await
        .map_err(session::internal)?;
    Ok(rows.into_iter().map(row_to_dto).collect())
}

#[cfg(feature = "server")]
async fn _running_timesheet() -> Result<Option<TimesheetDto>, ServerFnError> {
    use crate::session;

    let (user, workspace_id) = session::session_workspace().await?;
    let row = loom::tenant::timesheet::running(&workspace_id, &user.id)
        .await
        .map_err(session::internal)?;
    Ok(row.map(row_to_dto))
}

#[cfg(feature = "server")]
async fn _start_timesheet(
    project_id: Option<String>,
    activity_id: Option<String>,
    description: Option<String>,
    billable: bool,
) -> Result<TimesheetDto, ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::TIMESHEET_CREATE).await?;

    let r = loom::tenant::timesheet::start(
        &workspace_id,
        &user.id,
        project_id,
        activity_id,
        description,
        billable,
    )
    .await
    .map_err(session::internal)?;
    Ok(row_to_dto(r))
}

#[cfg(feature = "server")]
async fn _reassign_timesheet(
    timesheet_id: String,
    project_id: String,
    activity_id: String,
) -> Result<(), ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::TIMESHEET_UPDATE).await?;

    loom::tenant::timesheet::reassign(&workspace_id, &timesheet_id, project_id, activity_id)
        .await
        .map_err(session::internal)
}

#[cfg(feature = "server")]
async fn _update_timesheet(
    timesheet_id: String,
    description: Option<String>,
    billable: bool,
) -> Result<(), ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::TIMESHEET_UPDATE).await?;

    loom::tenant::timesheet::update(&workspace_id, &timesheet_id, description, billable)
        .await
        .map_err(session::internal)
}

#[cfg(feature = "server")]
async fn _stop_timesheet(timesheet_id: String) -> Result<(), ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    // Stopping is treated as a timesheet write operation.
    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::TIMESHEET_UPDATE).await?;

    loom::tenant::timesheet::stop(&workspace_id, &timesheet_id)
        .await
        .map_err(session::internal)
}

#[cfg(feature = "server")]
async fn _create_timesheet_manual(
    project_id: Option<String>,
    activity_id: Option<String>,
    start_time: String,
    end_time: String,
    description: Option<String>,
    billable: bool,
) -> Result<TimesheetDto, ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::TIMESHEET_CREATE).await?;

    let r = loom::tenant::timesheet::create_manual(
        &workspace_id,
        &user.id,
        project_id,
        activity_id,
        start_time,
        end_time,
        description,
        billable,
    )
    .await
    .map_err(session::internal)?;
    Ok(row_to_dto(r))
}

#[cfg(feature = "server")]
async fn _update_timesheet_time(
    timesheet_id: String,
    start_time: String,
    end_time: Option<String>,
) -> Result<(), ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    let (_user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&_user, permissions::TIMESHEET_UPDATE).await?;

    loom::tenant::timesheet::update_time(&workspace_id, &timesheet_id, start_time, end_time)
        .await
        .map_err(session::internal)
}

#[cfg(feature = "server")]
async fn _export_timesheet(timesheet_id: String) -> Result<(), ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::TIMESHEET_EXPORT).await?;

    loom::tenant::timesheet::export(&workspace_id, &timesheet_id)
        .await
        .map_err(session::internal)
}
