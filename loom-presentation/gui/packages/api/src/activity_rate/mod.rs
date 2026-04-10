use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivityRateDto {
    pub id: String,
    pub activity_id: String,
    pub user_id: Option<String>,
    /// Hourly rate in cents.
    pub hourly_rate: i64,
    pub internal_rate: Option<i64>,
}

#[get("/api/activity-rates")]
pub async fn list_activity_rates(
    activity_id: String,
) -> Result<Vec<ActivityRateDto>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _list_activity_rates(activity_id).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = activity_id;
        Ok(vec![])
    }
}

#[post("/api/activity-rates/set")]
pub async fn set_activity_rate(
    activity_id: String,
    hourly_rate: i64,
    internal_rate: Option<i64>,
) -> Result<ActivityRateDto, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _set_activity_rate(activity_id, hourly_rate, internal_rate).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (activity_id, hourly_rate, internal_rate);
        Err(ServerFnError::ServerError {
            message: "server only".into(),
            code: 500,
            details: None,
        })
    }
}

#[post("/api/activity-rates/remove")]
pub async fn remove_activity_rate(activity_id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _remove_activity_rate(activity_id).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = activity_id;
        Ok(())
    }
}

#[cfg(feature = "server")]
fn row_to_dto(
    r: loom::infrastructure::tenant::activity_rate::repositories::ActivityRateRow,
) -> ActivityRateDto {
    ActivityRateDto {
        id: r.id,
        activity_id: r.activity_id,
        user_id: r.user_id,
        hourly_rate: r.hourly_rate,
        internal_rate: r.internal_rate,
    }
}

#[cfg(feature = "server")]
async fn _list_activity_rates(activity_id: String) -> Result<Vec<ActivityRateDto>, ServerFnError> {
    use crate::session;

    let (_, workspace_id) = session::session_workspace().await?;
    loom::tenant::activity_rate::list_for_activity(&workspace_id, &activity_id)
        .await
        .map(|rows| rows.into_iter().map(row_to_dto).collect())
        .map_err(session::internal)
}

#[cfg(feature = "server")]
async fn _set_activity_rate(
    activity_id: String,
    hourly_rate: i64,
    internal_rate: Option<i64>,
) -> Result<ActivityRateDto, ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::RATE_MANAGE).await?;

    loom::tenant::activity_rate::set_default(&workspace_id, activity_id, hourly_rate, internal_rate)
        .await
        .map(row_to_dto)
        .map_err(session::internal)
}

#[cfg(feature = "server")]
async fn _remove_activity_rate(activity_id: String) -> Result<(), ServerFnError> {
    use crate::session;
    use loom::core::permissions;

    let (user, workspace_id) = session::session_workspace().await?;
    session::require_permission(&user, permissions::RATE_MANAGE).await?;

    loom::tenant::activity_rate::remove_default(&workspace_id, &activity_id)
        .await
        .map_err(session::internal)
}
