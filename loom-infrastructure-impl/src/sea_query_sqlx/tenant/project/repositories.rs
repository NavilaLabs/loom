use std::ops::Deref;

use async_trait::async_trait;
use eventually::aggregate::repository::{GetError, Getter, SaveError, Saver};
use eventually::serde::Json;
use eventually_any::snapshot::Repository;
use loom_core::tenant::project::{
    Project, ProjectEvent, ProjectId, ProjectRepository as ProjectRepositoryTrait,
};
use sqlx::{Row, any::AnyRow};

use crate::ConnectedTenantPool;

pub struct ProjectRepository {
    pool: ConnectedTenantPool,
    repository: Repository<Project, Json<Project>, Json<ProjectEvent>>,
}

impl Deref for ProjectRepository {
    type Target = Repository<Project, Json<Project>, Json<ProjectEvent>>;
    fn deref(&self) -> &Self::Target {
        &self.repository
    }
}

impl ProjectRepository {
    pub async fn from_pool(pool: ConnectedTenantPool) -> Result<Self, sqlx::migrate::MigrateError> {
        let repository =
            Repository::new(pool.as_ref().clone(), Json::default(), Json::default()).await?;
        Ok(Self { pool, repository })
    }

    pub async fn all(&self) -> Result<Vec<ProjectRow>, crate::Error> {
        let rows = sqlx::query(
            "SELECT id, customer_id, name, comment, order_number, visible, billable, \
             time_budget, money_budget, budget_is_monthly \
             FROM projections__projects ORDER BY name",
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        rows.into_iter().map(|r| Self::map_row(&r)).collect()
    }

    pub async fn by_customer(&self, customer_id: &str) -> Result<Vec<ProjectRow>, crate::Error> {
        let rows = sqlx::query(
            "SELECT id, customer_id, name, comment, order_number, visible, billable, \
             time_budget, money_budget, budget_is_monthly \
             FROM projections__projects WHERE customer_id = ? ORDER BY name",
        )
        .bind(customer_id)
        .fetch_all(self.pool.as_ref())
        .await?;
        rows.into_iter().map(|r| Self::map_row(&r)).collect()
    }

    fn map_row(row: &AnyRow) -> Result<ProjectRow, crate::Error> {
        Ok(ProjectRow {
            id: row.try_get("id")?,
            customer_id: row.try_get("customer_id")?,
            name: row.try_get("name")?,
            comment: row.try_get("comment")?,
            order_number: row.try_get("order_number")?,
            visible: bool_col(row, "visible"),
            billable: bool_col(row, "billable"),
            time_budget: row.try_get("time_budget")?,
            money_budget: row.try_get("money_budget")?,
            budget_is_monthly: bool_col(row, "budget_is_monthly"),
        })
    }
}

fn bool_col(row: &AnyRow, col: &str) -> bool {
    row.try_get::<bool, _>(col)
        .unwrap_or_else(|_| row.try_get::<i64, _>(col).map(|v| v != 0).unwrap_or(false))
}

#[derive(Debug, Clone)]
pub struct ProjectRow {
    pub id: String,
    pub customer_id: String,
    pub name: String,
    pub comment: Option<String>,
    pub order_number: Option<String>,
    pub visible: bool,
    pub billable: bool,
    pub time_budget: Option<i32>,
    pub money_budget: Option<i64>,
    pub budget_is_monthly: bool,
}

#[async_trait]
impl Getter<Project> for ProjectRepository {
    async fn get(&self, id: &ProjectId) -> Result<eventually::aggregate::Root<Project>, GetError> {
        self.repository.get(id).await
    }
}

#[async_trait]
impl Saver<Project> for ProjectRepository {
    async fn save(&self, root: &mut eventually::aggregate::Root<Project>) -> Result<(), SaveError> {
        self.repository.save(root).await
    }
}

impl ProjectRepositoryTrait for ProjectRepository {}
