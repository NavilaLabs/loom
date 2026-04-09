use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectRateDto {
    pub id: String,
    pub project_id: String,
    pub user_id: Option<String>,
    /// Hourly rate in cents (e.g. 10000 = €100.00).
    pub hourly_rate: i64,
    pub internal_rate: Option<i64>,
}

/// List all rates for a project.
#[get("/api/project-rates")]
pub async fn list_project_rates(
    project_id: String,
) -> Result<Vec<ProjectRateDto>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _list_project_rates(project_id).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = project_id;
        Ok(vec![])
    }
}

/// Set (or replace) the default hourly rate for a project.
#[post("/api/project-rates/set")]
pub async fn set_project_rate(
    project_id: String,
    hourly_rate: i64,
    internal_rate: Option<i64>,
) -> Result<ProjectRateDto, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _set_project_rate(project_id, hourly_rate, internal_rate).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (project_id, hourly_rate, internal_rate);
        Err(ServerFnError::ServerError {
            message: "server only".into(),
            code: 500,
            details: None,
        })
    }
}

/// Remove the default rate for a project.
#[post("/api/project-rates/remove")]
pub async fn remove_project_rate(project_id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _remove_project_rate(project_id).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = project_id;
        Ok(())
    }
}

#[cfg(feature = "server")]
async fn workspace_id_from_session() -> Result<String, ServerFnError> {
    use crate::auth::UserInfo;
    use dioxus::fullstack::extract;
    use tower_sessions::Session;
    let session: Session = extract().await?;
    let user: Option<UserInfo> = session
        .get("user")
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })?;
    user.and_then(|u| u.workspace_id)
        .ok_or_else(|| ServerFnError::ServerError {
            message: "not authenticated or no workspace".into(),
            code: 401,
            details: None,
        })
}

#[cfg(feature = "server")]
fn row_to_dto(
    r: loom::infrastructure::tenant::project_rate::repositories::ProjectRateRow,
) -> ProjectRateDto {
    ProjectRateDto {
        id: r.id,
        project_id: r.project_id,
        user_id: r.user_id,
        hourly_rate: r.hourly_rate,
        internal_rate: r.internal_rate,
    }
}

#[cfg(feature = "server")]
async fn _list_project_rates(
    project_id: String,
) -> Result<Vec<ProjectRateDto>, ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    loom::tenant::project_rate::list_for_project(&workspace_id, &project_id)
        .await
        .map(|rows| rows.into_iter().map(row_to_dto).collect())
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })
}

#[cfg(feature = "server")]
async fn _set_project_rate(
    project_id: String,
    hourly_rate: i64,
    internal_rate: Option<i64>,
) -> Result<ProjectRateDto, ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    loom::tenant::project_rate::set_default(&workspace_id, project_id, hourly_rate, internal_rate)
        .await
        .map(row_to_dto)
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })
}

#[cfg(feature = "server")]
async fn _remove_project_rate(project_id: String) -> Result<(), ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    loom::tenant::project_rate::remove_default(&workspace_id, &project_id)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })
}
