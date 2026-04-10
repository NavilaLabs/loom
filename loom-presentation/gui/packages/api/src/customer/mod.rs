use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerDto {
    pub id: String,
    pub name: String,
    pub comment: Option<String>,
    pub currency: String,
    pub timezone: String,
    pub country: Option<String>,
    pub visible: bool,
    pub time_budget: Option<i32>,
    pub money_budget: Option<i64>,
    pub budget_is_monthly: bool,
}

#[get("/api/customers")]
pub async fn list_customers() -> Result<Vec<CustomerDto>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _list_customers().await
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(vec![])
    }
}

#[post("/api/customers")]
pub async fn create_customer(
    name: String,
    currency: String,
    timezone: String,
) -> Result<CustomerDto, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _create_customer(name, currency, timezone).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (name, currency, timezone);
        Err(ServerFnError::ServerError {
            message: "server only".into(),
            code: 500,
            details: None,
        })
    }
}

#[post("/api/customers/budget")]
pub async fn set_customer_budget(
    id: String,
    time_budget: Option<i32>,
    money_budget: Option<i64>,
    budget_is_monthly: bool,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _set_customer_budget(id, time_budget, money_budget, budget_is_monthly).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (id, time_budget, money_budget, budget_is_monthly);
        Ok(())
    }
}

#[post("/api/customers/update")]
pub async fn update_customer(
    id: String,
    name: String,
    comment: Option<String>,
    currency: String,
    timezone: String,
    country: Option<String>,
    visible: bool,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _update_customer(id, name, comment, currency, timezone, country, visible).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (id, name, comment, currency, timezone, country, visible);
        Ok(())
    }
}

#[cfg(feature = "server")]
async fn _list_customers() -> Result<Vec<CustomerDto>, ServerFnError> {
    use crate::session;

    let (_, workspace_id) = session::session_workspace().await?;
    let rows = loom::tenant::customer::list(&workspace_id)
        .await
        .map_err(session::internal)?;
    Ok(rows
        .into_iter()
        .map(|r| CustomerDto {
            id: r.id,
            name: r.name,
            comment: r.comment,
            currency: r.currency,
            timezone: r.timezone,
            country: r.country,
            visible: r.visible,
            time_budget: r.time_budget,
            money_budget: r.money_budget,
            budget_is_monthly: r.budget_is_monthly,
        })
        .collect())
}

#[cfg(feature = "server")]
async fn _create_customer(
    name: String,
    currency: String,
    timezone: String,
) -> Result<CustomerDto, ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::CUSTOMER_CREATE).await?;

    let r = loom::tenant::customer::create(&workspace_id, name, currency, timezone)
        .await
        .map_err(session::internal)?;
    Ok(CustomerDto {
        id: r.id,
        name: r.name,
        comment: r.comment,
        currency: r.currency,
        timezone: r.timezone,
        country: r.country,
        visible: r.visible,
        time_budget: None,
        money_budget: None,
        budget_is_monthly: false,
    })
}

#[cfg(feature = "server")]
async fn _update_customer(
    id: String,
    name: String,
    comment: Option<String>,
    currency: String,
    timezone: String,
    country: Option<String>,
    visible: bool,
) -> Result<(), ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::CUSTOMER_UPDATE).await?;

    loom::tenant::customer::update(
        &workspace_id,
        &id,
        name,
        comment,
        currency,
        timezone,
        country,
        visible,
    )
    .await
    .map_err(session::internal)
}

#[cfg(feature = "server")]
async fn _set_customer_budget(
    id: String,
    time_budget: Option<i32>,
    money_budget: Option<i64>,
    budget_is_monthly: bool,
) -> Result<(), ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::CUSTOMER_UPDATE).await?;

    loom::tenant::customer::set_budget(
        &workspace_id,
        &id,
        time_budget,
        money_budget,
        budget_is_monthly,
    )
    .await
    .map_err(session::internal)
}
