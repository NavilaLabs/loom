use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagDto {
    pub id: String,
    pub name: String,
}

#[get("/api/tags")]
pub async fn list_tags() -> Result<Vec<TagDto>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _list_tags().await
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(vec![])
    }
}

#[get("/api/tags/for-timesheet")]
pub async fn list_timesheet_tags(timesheet_id: String) -> Result<Vec<TagDto>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _list_timesheet_tags(timesheet_id).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = timesheet_id;
        Ok(vec![])
    }
}

#[post("/api/tags")]
pub async fn create_tag(name: String) -> Result<TagDto, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _create_tag(name).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = name;
        Err(ServerFnError::ServerError {
            message: "server only".into(),
            code: 500,
            details: None,
        })
    }
}

#[post("/api/tags/rename")]
pub async fn rename_tag(id: String, name: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _rename_tag(id, name).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (id, name);
        Ok(())
    }
}

#[post("/api/tags/tag-timesheet")]
pub async fn tag_timesheet(tag_id: String, timesheet_id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _tag_timesheet(tag_id, timesheet_id).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (tag_id, timesheet_id);
        Ok(())
    }
}

#[post("/api/tags/untag-timesheet")]
pub async fn untag_timesheet(tag_id: String, timesheet_id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _untag_timesheet(tag_id, timesheet_id).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (tag_id, timesheet_id);
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
async fn _list_tags() -> Result<Vec<TagDto>, ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    loom::tenant::tag::list(&workspace_id)
        .await
        .map(|rows| rows.into_iter().map(|r| TagDto { id: r.id, name: r.name }).collect())
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })
}

#[cfg(feature = "server")]
async fn _list_timesheet_tags(timesheet_id: String) -> Result<Vec<TagDto>, ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    loom::tenant::tag::list_for_timesheet(&workspace_id, &timesheet_id)
        .await
        .map(|rows| rows.into_iter().map(|r| TagDto { id: r.id, name: r.name }).collect())
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })
}

#[cfg(feature = "server")]
async fn _create_tag(name: String) -> Result<TagDto, ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    loom::tenant::tag::create(&workspace_id, name)
        .await
        .map(|r| TagDto { id: r.id, name: r.name })
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })
}

#[cfg(feature = "server")]
async fn _rename_tag(id: String, name: String) -> Result<(), ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    loom::tenant::tag::rename(&workspace_id, &id, name)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })
}

#[cfg(feature = "server")]
async fn _tag_timesheet(tag_id: String, timesheet_id: String) -> Result<(), ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    loom::tenant::tag::tag_timesheet(&workspace_id, &tag_id, &timesheet_id)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })
}

#[cfg(feature = "server")]
async fn _untag_timesheet(tag_id: String, timesheet_id: String) -> Result<(), ServerFnError> {
    let workspace_id = workspace_id_from_session().await?;
    loom::tenant::tag::untag_timesheet(&workspace_id, &tag_id, &timesheet_id)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })
}
