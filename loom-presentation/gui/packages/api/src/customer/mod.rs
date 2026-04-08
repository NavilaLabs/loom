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
        Err(ServerFnError::ServerError { message: "server only".into(), code: 500, details: None })
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
async fn _list_customers() -> Result<Vec<CustomerDto>, ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    let rows = loom::tenant::customer::list(&workspace_id)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })?;
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
        })
        .collect())
}

#[cfg(feature = "server")]
async fn _create_customer(
    name: String,
    currency: String,
    timezone: String,
) -> Result<CustomerDto, ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    let r = loom::tenant::customer::create(&workspace_id, name, currency, timezone)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })?;
    Ok(CustomerDto {
        id: r.id,
        name: r.name,
        comment: r.comment,
        currency: r.currency,
        timezone: r.timezone,
        country: r.country,
        visible: r.visible,
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
    let workspace_id = workspace_id_from_session().await?;
    loom::tenant::customer::update(&workspace_id, &id, name, comment, currency, timezone, country, visible)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })
}
