use dioxus::prelude::*;

#[post("/api/login")]
pub async fn login(email: String, password: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "server")]
    {
        _login(email, password).await
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (email, password);
        Ok(String::new())
    }
}

#[cfg(feature = "server")]
async fn _login(email: String, password: String) -> Result<String, ServerFnError> {
    loom::auth::login_user(email, password)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: e.to_string(),
            code: 401,
            details: None,
        })
}
