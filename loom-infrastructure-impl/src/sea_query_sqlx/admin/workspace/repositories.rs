use std::{ops::Deref, str::FromStr};

use async_trait::async_trait;
use eventually::aggregate::repository::{GetError, Getter, SaveError, Saver};
use eventually::serde::Json;
use eventually_any::snapshot::Repository;
use loom_core::admin::workspace::{
    Workspace, WorkspaceEvent, WorkspaceId, WorkspaceRepository as WorkspaceRepositoryTrait,
    WorkspaceView,
};
use loom_infrastructure::query::{Query, RowToView};
use sea_query::{Alias, Condition, Expr, ExprTrait, Func, SelectStatement};
use sqlx::{Row, any::AnyRow, types::Uuid};

use crate::ConnectedAdminPool;

const TABLE: &str = "projections__workspaces";

pub struct WorkspaceRepository {
    database: ConnectedAdminPool,
    repository: Repository<Workspace, Json<Workspace>, Json<WorkspaceEvent>>,
}

impl Deref for WorkspaceRepository {
    type Target = Repository<Workspace, Json<Workspace>, Json<WorkspaceEvent>>;

    fn deref(&self) -> &Self::Target {
        &self.repository
    }
}

impl WorkspaceRepository {
    pub fn new(
        database: ConnectedAdminPool,
        repository: Repository<Workspace, Json<Workspace>, Json<WorkspaceEvent>>,
    ) -> Self {
        Self {
            database,
            repository,
        }
    }

    pub async fn from_pool(pool: ConnectedAdminPool) -> Result<Self, sqlx::migrate::MigrateError> {
        let repository =
            Repository::new(pool.as_ref().clone(), Json::default(), Json::default()).await?;
        Ok(Self {
            database: pool,
            repository,
        })
    }

    pub fn event_store(&self) -> &Repository<Workspace, Json<Workspace>, Json<WorkspaceEvent>> {
        &self.repository
    }

    /// Returns all (workspace_id, workspace_name) pairs the given user belongs to.
    pub async fn find_workspaces_for_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<(String, Option<String>)>, crate::Error> {
        let rows = sqlx::query(
            "SELECT DISTINCT w.id, w.name \
             FROM projections__workspaces w \
             INNER JOIN projections__workspace_user_roles r ON w.id = r.workspace_id \
             WHERE r.user_id = ?",
        )
        .bind(user_id)
        .fetch_all(self.database.as_ref())
        .await?;

        rows.into_iter()
            .map(|row| -> Result<_, crate::Error> {
                Ok((
                    row.try_get::<String, _>("id")?,
                    row.try_get::<Option<String>, _>("name")?,
                ))
            })
            .collect()
    }

    /// Returns the first workspace ID the given user belongs to, or `None`.
    pub async fn find_workspace_for_user(
        &self,
        user_id: &str,
    ) -> Result<Option<String>, crate::Error> {
        use sea_query::{Alias, Expr, ExprTrait};

        let statement = sea_query::Query::select()
            .expr(Expr::col(Alias::new("workspace_id")))
            .from(Alias::new("projections__workspace_user_roles"))
            .and_where(Expr::col(Alias::new("user_id")).eq(user_id))
            .limit(1)
            .to_owned();

        let (sql, arguments) = self.database.build_query(&statement);
        let row = sqlx::query_with(&sql, arguments)
            .fetch_optional(self.database.as_ref())
            .await?;

        row.map(|r| r.try_get::<String, _>(0usize).map_err(crate::Error::from))
            .transpose()
    }

    /// Fetch a `WorkspaceView` by string ID, avoiding the AnyPool UUID-type panic.
    pub async fn find_view_by_id(&self, id: &str) -> Result<Option<WorkspaceView>, crate::Error> {
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

    fn select(&self) -> SelectStatement {
        sea_query::Query::select()
            .expr(Expr::col(sea_query::Asterisk))
            .from(TABLE)
            .to_owned()
    }

    fn select_count(&self) -> SelectStatement {
        sea_query::Query::select()
            .expr(Func::count(Expr::col(sea_query::Asterisk)))
            .from(TABLE)
            .to_owned()
    }
}

impl RowToView<AnyRow> for WorkspaceRepository {
    type View = WorkspaceView;
    type Error = crate::Error;

    fn row_to_view(&self, row: AnyRow) -> Result<WorkspaceView, crate::Error> {
        let id: String = row.try_get("id")?;
        let id = Uuid::from_str(&id)?;
        let name: Option<String> = row.try_get("name")?;
        let timezone: String =
            row.try_get("timezone").unwrap_or_else(|_| "UTC".to_string());
        let date_format: String =
            row.try_get("date_format").unwrap_or_else(|_| "%Y-%m-%d".to_string());
        let currency: String =
            row.try_get("currency").unwrap_or_else(|_| "USD".to_string());
        let week_start: String =
            row.try_get("week_start").unwrap_or_else(|_| "monday".to_string());
        Ok(WorkspaceView::new_with_settings(
            id.into(),
            name,
            timezone,
            date_format,
            currency,
            week_start,
        ))
    }
}

#[async_trait]
impl Query<AnyRow> for WorkspaceRepository {
    type Filter = Condition;

    async fn get_one(&self, id: Uuid) -> Result<WorkspaceView, crate::Error> {
        self.get_one_by(Condition::all().add(Expr::col("id").eq(id)))
            .await
    }

    async fn find_one(&self, id: Uuid) -> Result<Option<WorkspaceView>, crate::Error> {
        self.find_one_by(Condition::all().add(Expr::col("id").eq(id)))
            .await
    }

    async fn get_one_by(&self, filter: Condition) -> Result<WorkspaceView, crate::Error> {
        let statement = self.select().cond_where(filter).to_owned();
        let (sql, arguments) = self.database.build_query(&statement);
        let row = sqlx::query_with(&sql, arguments)
            .fetch_one(self.database.as_ref())
            .await?;
        self.row_to_view(row)
    }

    async fn find_one_by(&self, filter: Condition) -> Result<Option<WorkspaceView>, crate::Error> {
        let statement = self.select().cond_where(filter).to_owned();
        let (sql, arguments) = self.database.build_query(&statement);
        let row = sqlx::query_with(&sql, arguments)
            .fetch_optional(self.database.as_ref())
            .await?;
        row.map(|r| self.row_to_view(r)).transpose()
    }

    async fn find_many(&self, ids: Vec<Uuid>) -> Result<Vec<WorkspaceView>, crate::Error> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        self.find_many_by(Condition::all().add(Expr::col("id").is_in(ids)))
            .await
    }

    async fn find_many_by(&self, filter: Condition) -> Result<Vec<WorkspaceView>, crate::Error> {
        let statement = self.select().cond_where(filter).to_owned();
        let (sql, arguments) = self.database.build_query(&statement);
        let rows = sqlx::query_with(&sql, arguments)
            .fetch_all(self.database.as_ref())
            .await?;
        rows.into_iter().map(|row| self.row_to_view(row)).collect()
    }

    async fn all(&self) -> Result<Vec<WorkspaceView>, crate::Error> {
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

#[async_trait]
impl Getter<Workspace> for WorkspaceRepository {
    async fn get(
        &self,
        id: &WorkspaceId,
    ) -> Result<eventually::aggregate::Root<Workspace>, GetError> {
        self.repository.get(id).await
    }
}

#[async_trait]
impl Saver<Workspace> for WorkspaceRepository {
    async fn save(
        &self,
        root: &mut eventually::aggregate::Root<Workspace>,
    ) -> Result<(), SaveError> {
        self.repository.save(root).await
    }
}

impl WorkspaceRepositoryTrait for WorkspaceRepository {}
