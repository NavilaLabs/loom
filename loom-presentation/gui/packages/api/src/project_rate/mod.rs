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
pub async fn list_project_rates(project_id: String) -> Result<Vec<ProjectRateDto>, ServerFnError> {
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
async fn _list_project_rates(project_id: String) -> Result<Vec<ProjectRateDto>, ServerFnError> {
    use crate::session;

    let (_, workspace_id) = session::session_workspace().await?;
    loom::tenant::project_rate::list_for_project(&workspace_id, &project_id)
        .await
        .map(|rows| rows.into_iter().map(row_to_dto).collect())
        .map_err(session::internal)
}

#[cfg(feature = "server")]
async fn _set_project_rate(
    project_id: String,
    hourly_rate: i64,
    internal_rate: Option<i64>,
) -> Result<ProjectRateDto, ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::RATE_MANAGE).await?;

    loom::tenant::project_rate::set_default(&workspace_id, project_id, hourly_rate, internal_rate)
        .await
        .map(row_to_dto)
        .map_err(session::internal)
}

#[cfg(feature = "server")]
async fn _remove_project_rate(project_id: String) -> Result<(), ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::RATE_MANAGE).await?;

    loom::tenant::project_rate::remove_default(&workspace_id, &project_id)
        .await
        .map_err(session::internal)
}
