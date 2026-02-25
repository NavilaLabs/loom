use std::marker::PhantomData;

use loom_infrastructure::ImplError;
use sea_orm::{ExprTrait, Value};
use sea_query::{Alias, Expr};

pub type ConnectedAdminPool = Provider<ScopeAdmin, StateConnected>;
pub type ConnectedTenantPool = Provider<ScopeTenant, StateConnected>;

#[derive(Debug)]
pub struct ScopeDefault;

#[derive(Debug)]
pub struct ScopeAdmin;

#[derive(Debug)]
pub struct ScopeTenant;

#[derive(Debug)]
pub struct StateConnected {
    pool: sqlx::AnyPool,
}

impl StateConnected {
    pub fn new(pool: sqlx::AnyPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug)]
pub struct StateDisconnected;

#[derive(Debug, PartialEq)]
pub enum DatabaseType {
    Postgres,
    Sqlite,
}

#[derive(Debug)]
pub struct Provider<Scope, State = StateDisconnected> {
    state: State,
    database_type: DatabaseType,
    _scope: PhantomData<Scope>,
}

impl<Scope, State> ImplError for Provider<Scope, State> {
    type Error = crate::Error;
}

impl<Scope> AsRef<sqlx::AnyPool> for Provider<Scope, StateConnected> {
    fn as_ref(&self) -> &sqlx::AnyPool {
        &self.state.pool
    }
}

impl<Scope, State> Provider<Scope, State> {
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

    pub fn cast_uuid(&self, uuid: String) -> Expr {
        if self.database_type == DatabaseType::Postgres {
            Expr::val(uuid).cast_as(Alias::new("uuid"))
        } else {
            Expr::val(uuid)
        }
    }

    pub fn cast_uuid_opt(&self, uuid: Option<String>) -> Expr {
        let val = match uuid {
            Some(s) => Value::String(Some(s)),
            None => Value::String(None),
        };

        if self.database_type == DatabaseType::Postgres {
            Expr::val(val).cast_as(Alias::new("uuid"))
        } else {
            Expr::val(val)
        }
    }

    pub fn cast_jsonb(&self, json: String) -> Expr {
        let val = if json == "{}".to_string() {
            Value::String(None)
        } else {
            Value::String(Some(json))
        };

        if self.database_type == DatabaseType::Postgres {
            Expr::val(val).cast_as(Alias::new("jsonb"))
        } else {
            Expr::val(val)
        }
    }

    pub fn cast_timestamp(&self, timestamp: String) -> Expr {
        if self.database_type == DatabaseType::Postgres {
            Expr::val(timestamp).cast_as(Alias::new("timestamp"))
        } else {
            Expr::val(timestamp)
        }
    }

    pub fn cast_timestamp_opt(&self, timestamp: Option<String>) -> Expr {
        let val = match timestamp {
            Some(s) => Value::String(Some(s)),
            None => Value::String(None),
        };

        if self.database_type == DatabaseType::Postgres {
            Expr::val(val).cast_as(Alias::new("timestamp"))
        } else {
            Expr::val(val)
        }
    }
}
