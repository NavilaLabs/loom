use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimesheetDto {
    pub id: String,
    pub user_id: String,
    pub project_id: String,
    pub activity_id: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration: Option<i32>,
    pub description: Option<String>,
    pub timezone: String,
    pub billable: bool,
    pub exported: bool,
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
    project_id: String,
    activity_id: String,
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
        Err(ServerFnError::ServerError { message: "server only".into(), code: 500, details: None })
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

#[cfg(feature = "server")]
async fn session_user() -> Result<crate::auth::UserInfo, ServerFnError> {
    use crate::auth::UserInfo;
    use dioxus::fullstack::extract;
    use tower_sessions::Session;

    let session: Session = extract().await?;
    let user: Option<UserInfo> = session.get("user").await.map_err(|e| ServerFnError::ServerError {
        message: e.to_string(),
        code: 500,
        details: None,
    })?;
    user.ok_or_else(|| ServerFnError::ServerError {
        message: "not authenticated".into(),
        code: 401,
        details: None,
    })
}

#[cfg(feature = "server")]
fn row_to_dto(r: loom::infrastructure::tenant::timesheet::repositories::TimesheetRow) -> TimesheetDto {
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
    }
}

#[cfg(feature = "server")]
async fn _list_timesheets() -> Result<Vec<TimesheetDto>, ServerFnError> {
    let user = session_user().await?;
    let workspace_id = user.workspace_id.ok_or_else(|| ServerFnError::ServerError {
        message: "no workspace".into(),
        code: 401,
        details: None,
    })?;
    let rows = loom::tenant::timesheet::recent(&workspace_id, &user.id)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })?;
    Ok(rows.into_iter().map(row_to_dto).collect())
}

#[cfg(feature = "server")]
async fn _running_timesheet() -> Result<Option<TimesheetDto>, ServerFnError> {
    let user = session_user().await?;
    let workspace_id = user.workspace_id.ok_or_else(|| ServerFnError::ServerError {
        message: "no workspace".into(),
        code: 401,
        details: None,
    })?;
    let row = loom::tenant::timesheet::running(&workspace_id, &user.id)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })?;
    Ok(row.map(row_to_dto))
}

#[cfg(feature = "server")]
async fn _start_timesheet(
    project_id: String,
    activity_id: String,
    description: Option<String>,
    billable: bool,
) -> Result<TimesheetDto, ServerFnError> {
    let user = session_user().await?;
    let workspace_id = user.workspace_id.ok_or_else(|| ServerFnError::ServerError {
        message: "no workspace".into(),
        code: 401,
        details: None,
    })?;
    let r = loom::tenant::timesheet::start(
        &workspace_id,
        &user.id,
        project_id,
        activity_id,
        description,
        billable,
    )
    .await
    .map_err(|e| ServerFnError::ServerError {
        message: e.to_string(),
        code: 500,
        details: None,
    })?;
    Ok(row_to_dto(r))
}

#[cfg(feature = "server")]
async fn _stop_timesheet(timesheet_id: String) -> Result<(), ServerFnError> {
    let user = session_user().await?;
    let workspace_id = user.workspace_id.ok_or_else(|| ServerFnError::ServerError {
        message: "no workspace".into(),
        code: 401,
        details: None,
    })?;
    loom::tenant::timesheet::stop(&workspace_id, &timesheet_id)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })
}
