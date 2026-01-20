mod connection;

mod initiate;
use sqlx::{Any, pool::PoolConnection};

use std::marker::PhantomData;

use crate::config;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::Error),
    #[error("SQLx error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

type AcquiredConnection = PoolConnection<Any>;

#[derive(Debug)]
pub struct ScopeDefault;

#[derive(Debug)]
pub struct ScopeAdmin;

#[derive(Debug)]
pub struct ScopeTenant;

#[derive(Debug)]
pub struct StateConnected;

#[derive(Debug)]
pub struct StateDisconnected;

#[derive(Debug)]
pub struct Pool<Scope, State = StateDisconnected> {
    pool: Option<sqlx::AnyPool>,
    _scope: PhantomData<Scope>,
    _state: PhantomData<State>,
}

impl AsRef<sqlx::AnyPool> for Pool<ScopeDefault, StateConnected> {
    fn as_ref(&self) -> &sqlx::AnyPool {
        self.pool.as_ref().unwrap()
    }
}

// #[async_trait::async_trait]
// impl database::initialize::Initialize<sqlx::PgPool> for Database<StateConnected> {
//     async fn initialize_admin_database(
//         &self,
//         pool: &sqlx::PgPool,
//     ) -> Result<(), <Pool as database::Connection<sqlx::PgPool>>::Error> {
//         let database_name = config::CONFIG
//             .get_database()
//             .get_databases()
//             .get_admin()
//             .get_name();

//         let query = format!(r#"CREATE DATABASE "{}""#, database_name);
//         sqlx::query(&query).execute(pool).await?;

//         Ok(())
//     }

//     async fn initialize_tenant_database(
//         &self,
//         pool: &sqlx::PgPool,
//     ) -> Result<(), <Pool as database::Connection<sqlx::PgPool>>::Error> {
//         let template_name = config::CONFIG
//             .get_database()
//             .get_databases()
//             .get_tenant()
//             .get_name_prefix();

//         let query = format!(r#"CREATE DATABASE "{}_template""#, template_name);
//         sqlx::query(&query).execute(pool).await?;

//         Ok(())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use infrastructure::database::{Connection, Initialize};

//     #[tokio::test]
//     async fn test_initialize_database() {
//         let db = super::Database;

//         // db.initialize_database(&db).await.unwrap();
//         todo!()
//     }

//     #[tokio::test]
//     async fn test_establish_admin_connection() {
//         let db = super::Database;

//         db.establish_admin_connection().await.unwrap();
//     }
// }
