use dioxus::prelude::*;

/// TODO: This should not be handled here. Instead it could be a separate binary to be run separately from the server as a standalone process.
#[post("/api/database/migrate")]
pub async fn migrate_database() -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        _migrate_database().await
    }
    #[cfg(not(feature = "server"))]
    {
        Ok(())
    }
}

#[cfg(feature = "server")]
async fn _migrate_database() -> Result<(), ServerFnError> {
    use dioxus::fullstack::extract;
    use loom::{
        authorization::AuthorizationService,
        infrastructure::{
            database::{Initializer, SqliteInitializationStrategy},
            Pool,
        },
        Migrate,
    };
    use tower_sessions::Session;

    // Require admin — only admins may trigger migrations.
    let session: Session = extract().await?;
    let user: Option<crate::auth::UserInfo> =
        session
            .get("user")
            .await
            .map_err(|e| ServerFnError::ServerError {
                message: e.to_string(),
                code: 500,
                details: None,
            })?;

    let user = user.ok_or_else(|| ServerFnError::ServerError {
        message: "unauthorized".to_string(),
        code: 401,
        details: None,
    })?;

    let current_user = loom::auth::CurrentUser {
        id: user.id,
        email: user.email,
    };

    AuthorizationService::require_admin(&current_user)
        .await
        .map_err(|_| ServerFnError::ServerError {
            message: "forbidden".to_string(),
            code: 403,
            details: None,
        })?;

    let default_pool =
        Pool::connect_default()
            .await
            .map_err(|error| ServerFnError::ServerError {
                message: error.to_string(),
                code: 500,
                details: None,
            })?;
    let initializer = Initializer::new(SqliteInitializationStrategy);
    let _ = initializer
        .initialize_admin(&default_pool)
        .await
        .map_err(|error| ServerFnError::ServerError {
            message: error.to_string(),
            code: 500,
            details: None,
        })?;

    let pool = Pool::connect_admin()
        .await
        .map_err(|error| ServerFnError::ServerError {
            message: error.to_string(),
            code: 500,
            details: None,
        })?;
    let _ = pool
        .migrate_database()
        .await
        .map_err(|error| ServerFnError::ServerError {
            message: error.to_string(),
            code: 500,
            details: None,
        })?;

    Ok(())
}
