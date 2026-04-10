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
    pub time_budget: Option<i32>,
    pub money_budget: Option<i64>,
    pub budget_is_monthly: bool,
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
        Err(ServerFnError::ServerError {
            message: "server only".into(),
            code: 500,
            details: None,
        })
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

#[post("/api/projects/budget")]
pub async fn set_project_budget(
    id: String,
    time_budget: Option<i32>,
    money_budget: Option<i64>,
    budget_is_monthly: bool,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _set_project_budget(id, time_budget, money_budget, budget_is_monthly).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (id, time_budget, money_budget, budget_is_monthly);
        Ok(())
    }
}

#[cfg(feature = "server")]
async fn _list_projects() -> Result<Vec<ProjectDto>, ServerFnError> {
    use crate::session;

    let (_, workspace_id) = session::session_workspace().await?;
    let rows = loom::tenant::project::list(&workspace_id)
        .await
        .map_err(session::internal)?;
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
            time_budget: r.time_budget,
            money_budget: r.money_budget,
            budget_is_monthly: r.budget_is_monthly,
        })
        .collect())
}

#[cfg(feature = "server")]
async fn _create_project(customer_id: String, name: String) -> Result<ProjectDto, ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::PROJECT_CREATE).await?;

    let r = loom::tenant::project::create(&workspace_id, customer_id, name)
        .await
        .map_err(session::internal)?;
    Ok(ProjectDto {
        id: r.id,
        customer_id: r.customer_id,
        name: r.name,
        comment: r.comment,
        order_number: r.order_number,
        visible: r.visible,
        billable: r.billable,
        time_budget: None,
        money_budget: None,
        budget_is_monthly: false,
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
    use crate::session;
    use loom::core::permissions;

    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::PROJECT_UPDATE).await?;

    loom::tenant::project::update(
        &workspace_id,
        &id,
        name,
        comment,
        order_number,
        visible,
        billable,
    )
    .await
    .map_err(session::internal)
}

#[cfg(feature = "server")]
async fn _set_project_budget(
    id: String,
    time_budget: Option<i32>,
    money_budget: Option<i64>,
    budget_is_monthly: bool,
) -> Result<(), ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::PROJECT_UPDATE).await?;

    loom::tenant::project::set_budget(
        &workspace_id,
        &id,
        time_budget,
        money_budget,
        budget_is_monthly,
    )
    .await
    .map_err(session::internal)
}
