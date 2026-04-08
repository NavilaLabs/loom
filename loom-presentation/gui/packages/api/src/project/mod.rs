use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectDto {
    pub id: String,
    pub customer_id: String,
    pub name: String,
    pub comment: Option<String>,
    pub order_number: Option<String>,
    pub visible: bool,
    pub billable: bool,
}

#[get("/api/projects")]
pub async fn list_projects() -> Result<Vec<ProjectDto>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _list_projects().await
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(vec![])
    }
}

#[post("/api/projects")]
pub async fn create_project(
    customer_id: String,
    name: String,
) -> Result<ProjectDto, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _create_project(customer_id, name).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (customer_id, name);
        Err(ServerFnError::ServerError { message: "server only".into(), code: 500, details: None })
    }
}

#[post("/api/projects/update")]
pub async fn update_project(
    id: String,
    name: String,
    comment: Option<String>,
    order_number: Option<String>,
    visible: bool,
    billable: bool,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _update_project(id, name, comment, order_number, visible, billable).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (id, name, comment, order_number, visible, billable);
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
async fn _list_projects() -> Result<Vec<ProjectDto>, ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    let rows = loom::tenant::project::list(&workspace_id)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })?;
    Ok(rows
        .into_iter()
        .map(|r| ProjectDto {
            id: r.id,
            customer_id: r.customer_id,
            name: r.name,
            comment: r.comment,
            order_number: r.order_number,
            visible: r.visible,
            billable: r.billable,
        })
        .collect())
}

#[cfg(feature = "server")]
async fn _create_project(
    customer_id: String,
    name: String,
) -> Result<ProjectDto, ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    let r = loom::tenant::project::create(&workspace_id, customer_id, name)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })?;
    Ok(ProjectDto {
        id: r.id,
        customer_id: r.customer_id,
        name: r.name,
        comment: r.comment,
        order_number: r.order_number,
        visible: r.visible,
        billable: r.billable,
    })
}

#[cfg(feature = "server")]
async fn _update_project(
    id: String,
    name: String,
    comment: Option<String>,
    order_number: Option<String>,
    visible: bool,
    billable: bool,
) -> Result<(), ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    loom::tenant::project::update(&workspace_id, &id, name, comment, order_number, visible, billable)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })
}
