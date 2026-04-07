use dioxus::prelude::*;

#[post("/api/setup")]
pub async fn setup(
    username: String,
    email: String,
    password: String,
    workspace_name: String,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _setup(username, email, password, workspace_name).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (username, email, password, workspace_name);
        Ok(())
    }
}

#[cfg(feature = "server")]
async fn _setup(
    username: String,
    email: String,
    password: String,
    workspace_name: String,
) -> Result<(), ServerFnError> {
    loom::setup::setup_application(username, email, password, workspace_name)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 500,
            details: None,
        })
}
