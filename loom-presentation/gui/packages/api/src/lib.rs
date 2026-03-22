//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;
use loom_infrastructure::database::Migrate;
use loom_infrastructure_impl::Pool;

/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[post("/api/database/migrate")]
pub async fn migrate_database() -> Result<(), ServerFnError> {
    let _ = Pool::connect_admin()
        .await
        .map(
            async move |pool: Pool<
                loom_infrastructure_impl::ScopeAdmin,
                loom_infrastructure_impl::StateConnected,
            >| pool.migrate_database().await,
        )
        .map_err(|error| ServerFnError::ServerError {
            message: error.to_string(),
            code: 500,
            details: None,
        })?;

    Ok(())
}
