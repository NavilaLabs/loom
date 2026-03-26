use dioxus::prelude::*;

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
    use loom::{
        infrastructure::{
            database::{Initializer, SqliteInitializationStrategy},
            Pool,
        },
        Migrate,
    };

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
