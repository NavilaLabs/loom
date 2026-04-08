use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivityDto {
    pub id: String,
    pub project_id: Option<String>,
    pub name: String,
    pub comment: Option<String>,
    pub visible: bool,
    pub billable: bool,
}

#[get("/api/activities")]
pub async fn list_activities() -> Result<Vec<ActivityDto>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _list_activities().await
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(vec![])
    }
}

#[post("/api/activities")]
pub async fn create_activity(
    project_id: Option<String>,
    name: String,
) -> Result<ActivityDto, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _create_activity(project_id, name).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (project_id, name);
        Err(ServerFnError::ServerError { message: "server only".into(), code: 500, details: None })
    }
}

#[post("/api/activities/update")]
pub async fn update_activity(
    id: String,
    name: String,
    comment: Option<String>,
    visible: bool,
    billable: bool,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _update_activity(id, name, comment, visible, billable).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (id, name, comment, visible, billable);
        Ok(())
    }
}

#[cfg(feature = "server")]
async fn workspace_id_from_session() -> Result<String, ServerFnError> {
    use crate::auth::UserInfo;
    use dioxus::fullstack::extract;
    use tower_sessions::Session;

    let session: Session = extract().await?;
    let user: Option<UserInfo> = session.get("user").await.map_err(|e| ServerFnError::ServerError {
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
async fn _list_activities() -> Result<Vec<ActivityDto>, ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    let rows = loom::tenant::activity::list(&workspace_id)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })?;
    Ok(rows
        .into_iter()
        .map(|r| ActivityDto {
            id: r.id,
            project_id: r.project_id,
            name: r.name,
            comment: r.comment,
            visible: r.visible,
            billable: r.billable,
        })
        .collect())
}

#[cfg(feature = "server")]
async fn _create_activity(
    project_id: Option<String>,
    name: String,
) -> Result<ActivityDto, ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    let r = loom::tenant::activity::create(&workspace_id, project_id, name)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })?;
    Ok(ActivityDto {
        id: r.id,
        project_id: r.project_id,
        name: r.name,
        comment: r.comment,
        visible: r.visible,
        billable: r.billable,
    })
}

#[cfg(feature = "server")]
async fn _update_activity(
    id: String,
    name: String,
    comment: Option<String>,
    visible: bool,
    billable: bool,
) -> Result<(), ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    loom::tenant::activity::update(&workspace_id, &id, name, comment, visible, billable)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })
}
