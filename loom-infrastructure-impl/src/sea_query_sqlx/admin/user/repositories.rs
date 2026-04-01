use std::{ops::Deref, str::FromStr};

use async_trait::async_trait;
use eventually::serde::Json;
use eventually_any::snapshot::Repository;
use loom_core::admin::user::{User, UserEvent, UserView};
use loom_infrastructure::query::{Query, RowToView};
use sea_query::{Condition, Expr, ExprTrait, Func, SelectStatement};
use sqlx::{Row, any::AnyRow, types::Uuid};

use crate::ConnectedAdminPool;

const TABLE: &str = "projections__users";

pub struct UserRepository {
    database: ConnectedAdminPool,
    repository: Repository<User, Json<User>, Json<UserEvent>>,
}

impl Deref for UserRepository {
    type Target = Repository<User, Json<User>, Json<UserEvent>>;

    fn deref(&self) -> &Self::Target {
        &self.repository
    }
}

impl UserRepository {
    pub fn new(
        database: ConnectedAdminPool,
        repository: Repository<User, Json<User>, Json<UserEvent>>,
    ) -> Self {
        Self {
            database,
            repository,
        }
    }

    pub fn event_store(&self) -> &Repository<User, Json<User>, Json<UserEvent>> {
        &self.repository
    }

    /// Returns `(user_id, email, password_hash)` for the given email — intended
    /// only for authentication flows, not general display.
    pub async fn find_credentials_by_email(
        &self,
        email: &str,
    ) -> Result<Option<(String, String, String)>, crate::Error> {
        let statement = self
            .select()
            .columns(["id", "email", "password_hash"])
            .cond_where(Condition::all().add(Expr::col("email").eq(email)))
            .to_owned();
        let (sql, arguments) = self.database.build_query(&statement);

        let row = sqlx::query_with(&sql, arguments)
            .fetch_optional(self.database.as_ref())
            .await?;

        row.map(|r| {
            let id: String = r.try_get("id")?;
            let email: String = r.try_get("email")?;
            let hash: String = r.try_get("password_hash")?;
            Ok((id, email, hash))
        })
        .transpose()
    }

    pub async fn has_at_least_one_user(&self) -> Result<bool, crate::Error> {
        Ok(self.count().await? > 0)
    }

    fn select(&self) -> SelectStatement {
        sea_query::Query::select().from(TABLE).to_owned()
    }

    fn select_count(&self) -> SelectStatement {
        sea_query::Query::select()
            .expr(Func::count(Expr::col(sea_query::Asterisk)))
            .from(TABLE)
            .to_owned()
    }
}

impl RowToView<AnyRow> for UserRepository {
    type View = UserView;
    type Error = crate::Error;

    fn row_to_view(&self, row: AnyRow) -> Result<UserView, crate::Error> {
        let id: String = row.try_get("id")?;
        let id = Uuid::from_str(&id)?;
        let name: String = row.try_get("name")?;
        let email: String = row.try_get("email")?;

        Ok(UserView::new(id.into(), name, email))
    }
}

#[async_trait]
impl Query<AnyRow> for UserRepository {
    type Filter = Condition;

    async fn get_one(&self, id: Uuid) -> Result<UserView, crate::Error> {
        self.get_one_by(Condition::all().add(Expr::col("id").eq(id)))
            .await
    }

    async fn find_one(&self, id: Uuid) -> Result<Option<UserView>, crate::Error> {
        self.find_one_by(Condition::all().add(Expr::col("id").eq(id)))
            .await
    }

    async fn get_one_by(&self, filter: Condition) -> Result<UserView, crate::Error> {
        let statement = self.select().cond_where(filter).to_owned();
        let (sql, arguments) = self.database.build_query(&statement);

        let row = sqlx::query_with(&sql, arguments)
            .fetch_one(self.database.as_ref())
            .await?;

        self.row_to_view(row)
    }

    async fn find_one_by(&self, filter: Condition) -> Result<Option<UserView>, crate::Error> {
        let statement = self.select().cond_where(filter).to_owned();
        let (sql, arguments) = self.database.build_query(&statement);

        let row = sqlx::query_with(&sql, arguments)
            .fetch_optional(self.database.as_ref())
            .await?;

        row.map(|r| self.row_to_view(r)).transpose()
    }

    async fn find_many(&self, ids: Vec<Uuid>) -> Result<Vec<UserView>, crate::Error> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        self.find_many_by(Condition::all().add(Expr::col("id").is_in(ids)))
            .await
    }

    async fn find_many_by(&self, filter: Condition) -> Result<Vec<UserView>, crate::Error> {
        let statement = self.select().cond_where(filter).to_owned();
        let (sql, arguments) = self.database.build_query(&statement);

        let rows = sqlx::query_with(&sql, arguments)
            .fetch_all(self.database.as_ref())
            .await?;

        rows.into_iter().map(|row| self.row_to_view(row)).collect()
    }

    async fn all(&self) -> Result<Vec<UserView>, crate::Error> {
        let (sql, arguments) = self.database.build_query(&self.select());

        let rows = sqlx::query_with(&sql, arguments)
            .fetch_all(self.database.as_ref())
            .await?;

        rows.into_iter().map(|row| self.row_to_view(row)).collect()
    }

    async fn count_by(&self, filter: Condition) -> Result<u64, crate::Error> {
        let statement = self.select_count().cond_where(filter).to_owned();
        let (sql, arguments) = self.database.build_query(&statement);

        let row = sqlx::query_with(&sql, arguments)
            .fetch_one(self.database.as_ref())
            .await?;

        let n: i64 = row.try_get(0)?;
        Ok(n as u64)
    }

    async fn count(&self) -> Result<u64, crate::Error> {
        let (sql, arguments) = self.database.build_query(&self.select_count());

        let row = sqlx::query_with(&sql, arguments)
            .fetch_one(self.database.as_ref())
            .await?;

        let n: i64 = row.try_get(0)?;
        Ok(n as u64)
    }
}
