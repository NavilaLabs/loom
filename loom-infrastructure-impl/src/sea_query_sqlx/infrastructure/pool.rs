use std::{fmt::Display, marker::PhantomData};

use loom_infrastructure::ImplError;
use url::Url;

pub type ConnectedAdminPool = Pool<ScopeAdmin, StateConnected>;
pub type ConnectedTenantPool = Pool<ScopeTenant, StateConnected>;

#[derive(Debug, Clone)]
pub struct ScopeDefault;

#[derive(Debug, Clone)]
pub struct ScopeAdmin;

#[derive(Debug, Clone)]
pub struct ScopeTenant;

#[derive(Debug, Clone)]
pub struct StateConnected {
    pool: sqlx::AnyPool,
}

impl StateConnected {
    pub fn new(pool: sqlx::AnyPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, Clone)]
pub struct StateDisconnected;

#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseType {
    Postgres,
    Sqlite,
}

impl Display for DatabaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseType::Postgres => write!(f, "postgres"),
            DatabaseType::Sqlite => write!(f, "sqlite"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pool<Scope, State = StateDisconnected> {
    state: State,
    database_type: DatabaseType,
    _scope: PhantomData<Scope>,
}

impl<Scope, State> ImplError for Pool<Scope, State> {
    type Error = crate::Error;
}

impl<Scope> AsRef<sqlx::AnyPool> for Pool<Scope, StateConnected> {
    fn as_ref(&self) -> &sqlx::AnyPool {
        &self.state.pool
    }
}

impl<Scope, State> Pool<Scope, State> {
    pub fn new(state: State, database_type: DatabaseType) -> Self {
        Self {
            state,
            database_type,
            _scope: PhantomData,
        }
    }

    pub fn get_database_type(&self) -> &DatabaseType {
        &self.database_type
    }
}

impl<Scope> Pool<Scope, StateConnected> {
    pub fn into_pool(self) -> sqlx::AnyPool {
        self.state.pool
    }

    pub fn get_uri(&self) -> Url {
        self.state.pool.connect_options().database_url.clone()
    }
}
