use std::{ops::Deref, str::FromStr};

use async_trait::async_trait;
use eventually::aggregate::repository::{GetError, Getter, SaveError, Saver};
use eventually::serde::Json;
use eventually_any::snapshot::Repository;
use loom_core::admin::user::{
    User, UserEvent, UserId, UserRepository as UserRepositoryTrait, UserView,
};
use loom_infrastructure::query::{Query, RowToView};
use sea_query::{Alias, Condition, Expr, ExprTrait, Func, SelectStatement};
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
    #[must_use]
    pub const fn new(
        database: ConnectedAdminPool,
        repository: Repository<User, Json<User>, Json<UserEvent>>,
    ) -> Self {
        Self {
            database,
            repository,
        }
    }

    /// # Errors
    ///
    /// Returns an error if the event store repository cannot be initialized.
    pub async fn from_pool(pool: ConnectedAdminPool) -> Result<Self, sqlx::migrate::MigrateError> {
        let repository =
            Repository::new(pool.as_ref().clone(), Json::default(), Json::default()).await?;
        Ok(Self {
            database: pool,
            repository,
        })
    }

    #[must_use]
    pub const fn event_store(&self) -> &Repository<User, Json<User>, Json<UserEvent>> {
        &self.repository
    }

    /// Returns `(user_id, email, password)` for the given email — intended
    /// only for authentication flows, not general display.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub async fn find_credentials_by_email(
        &self,
        email: &str,
    ) -> Result<Option<(String, String, String)>, crate::Error> {
        // Build an explicit SELECT with named expressions to avoid any
        // wildcard-expansion quirk in sea-query with AnyPool.
        let statement = sea_query::Query::select()
            .expr(Expr::col(Alias::new("id")))
            .expr(Expr::col(Alias::new("email")))
            .expr(Expr::col(Alias::new("password")))
            .from(Alias::new(TABLE))
            .and_where(Expr::col(Alias::new("email")).eq(email))
            .to_owned();
        let (sql, arguments) = self.database.build_query(&statement);

        tracing::debug!(sql = %sql, "find_credentials_by_email");

        let row = sqlx::query_with(&sql, arguments)
            .fetch_optional(self.database.as_ref())
            .await?;

        row.map(|r| {
            // Use positional indices — immune to column-name aliasing by the driver.
            let id: String = r.try_get(0usize)?;
            let email: String = r.try_get(1usize)?;
            let hash: String = r.try_get(2usize)?;
            Ok((id, email, hash))
        })
        .transpose()
    }

    /// # Errors
    ///
    /// Returns an error if the database count query fails.
    pub async fn has_at_least_one_user(&self) -> Result<bool, crate::Error> {
        Ok(self.count().await? > 0)
    }

    /// Fetch a `UserView` by string ID, avoiding the `AnyPool` UUID-type panic.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub async fn find_view_by_id(&self, id: &str) -> Result<Option<UserView>, crate::Error> {
        let statement = sea_query::Query::select()
            .expr(Expr::col(sea_query::Asterisk))
            .from(Alias::new(TABLE))
            .and_where(Expr::col(Alias::new("id")).eq(id))
            .to_owned();
        let (sql, arguments) = self.database.build_query(&statement);
        let row = sqlx::query_with(&sql, arguments)
            .fetch_optional(self.database.as_ref())
            .await?;
        row.map(|r| self.row_to_view(r)).transpose()
    }

    #[allow(clippy::unused_self)]
    fn select(&self) -> SelectStatement {
        sea_query::Query::select()
            .expr(Expr::col(sea_query::Asterisk))
            .from(TABLE)
            .to_owned()
    }

    #[allow(clippy::unused_self)]
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
        let timezone: String = row
            .try_get("timezone")
            .unwrap_or_else(|_| "UTC".to_string());
        let date_format: String = row
            .try_get("date_format")
            .unwrap_or_else(|_| "%Y-%m-%d".to_string());
        let language: String = row.try_get("language").unwrap_or_else(|_| "en".to_string());

        Ok(UserView::new_with_settings(
            id.into(),
            name,
            email,
            timezone,
            date_format,
            language,
        ))
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
        #[allow(clippy::cast_sign_loss)]
        Ok(n as u64)
    }

    async fn count(&self) -> Result<u64, crate::Error> {
        let (sql, arguments) = self.database.build_query(&self.select_count());

        let row = sqlx::query_with(&sql, arguments)
            .fetch_one(self.database.as_ref())
            .await?;

        let n: i64 = row.try_get(0)?;
        #[allow(clippy::cast_sign_loss)]
        Ok(n as u64)
    }
}

#[async_trait]
impl Getter<User> for UserRepository {
    async fn get(&self, id: &UserId) -> Result<eventually::aggregate::Root<User>, GetError> {
        self.repository.get(id).await
    }
}

#[async_trait]
impl Saver<User> for UserRepository {
    async fn save(&self, root: &mut eventually::aggregate::Root<User>) -> Result<(), SaveError> {
        self.repository.save(root).await
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    type Error = crate::Error;

    async fn find_credentials_by_email(
        &self,
        email: &str,
    ) -> Result<Option<(String, String, String)>, Self::Error> {
        // Calls the inherent method defined above; inherent methods take
        // priority over trait methods in method resolution, so this is not recursive.
        self.find_credentials_by_email(email).await
    }
}
