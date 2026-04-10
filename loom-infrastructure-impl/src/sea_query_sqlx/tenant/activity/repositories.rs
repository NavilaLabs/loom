use std::ops::Deref;

use async_trait::async_trait;
use eventually::aggregate::repository::{GetError, Getter, SaveError, Saver};
use eventually::serde::Json;
use eventually_any::snapshot::Repository;
use loom_core::tenant::activity::{
    Activity, ActivityEvent, ActivityId, ActivityRepository as ActivityRepositoryTrait,
};
use sqlx::{Row, any::AnyRow};

use crate::ConnectedTenantPool;

pub struct ActivityRepository {
    pool: ConnectedTenantPool,
    repository: Repository<Activity, Json<Activity>, Json<ActivityEvent>>,
}

impl Deref for ActivityRepository {
    type Target = Repository<Activity, Json<Activity>, Json<ActivityEvent>>;
    fn deref(&self) -> &Self::Target {
        &self.repository
    }
}

impl ActivityRepository {
    pub async fn from_pool(pool: ConnectedTenantPool) -> Result<Self, sqlx::migrate::MigrateError> {
        let repository =
            Repository::new(pool.as_ref().clone(), Json::default(), Json::default()).await?;
        Ok(Self { pool, repository })
    }

    pub async fn all(&self) -> Result<Vec<ActivityRow>, crate::Error> {
        let rows = sqlx::query(
            "SELECT id, project_id, name, comment, visible, billable \
             FROM projections__activities ORDER BY name",
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        rows.into_iter().map(|r| Self::map_row(&r)).collect()
    }

    pub async fn by_project(&self, project_id: &str) -> Result<Vec<ActivityRow>, crate::Error> {
        let rows = sqlx::query(
            "SELECT id, project_id, name, comment, visible, billable \
             FROM projections__activities WHERE project_id = ? OR project_id IS NULL ORDER BY name",
        )
        .bind(project_id)
        .fetch_all(self.pool.as_ref())
        .await?;
        rows.into_iter().map(|r| Self::map_row(&r)).collect()
    }

    fn map_row(row: &AnyRow) -> Result<ActivityRow, crate::Error> {
        Ok(ActivityRow {
            id: row.try_get("id")?,
            project_id: row.try_get("project_id")?,
            name: row.try_get("name")?,
            comment: row.try_get("comment")?,
            visible: bool_col(row, "visible"),
            billable: bool_col(row, "billable"),
        })
    }
}

fn bool_col(row: &AnyRow, col: &str) -> bool {
    row.try_get::<bool, _>(col)
        .unwrap_or_else(|_| row.try_get::<i64, _>(col).map(|v| v != 0).unwrap_or(false))
}

#[derive(Debug, Clone)]
pub struct ActivityRow {
    pub id: String,
    pub project_id: Option<String>,
    pub name: String,
    pub comment: Option<String>,
    pub visible: bool,
    pub billable: bool,
}

#[async_trait]
impl Getter<Activity> for ActivityRepository {
    async fn get(
        &self,
        id: &ActivityId,
    ) -> Result<eventually::aggregate::Root<Activity>, GetError> {
        self.repository.get(id).await
    }
}

#[async_trait]
impl Saver<Activity> for ActivityRepository {
    async fn save(
        &self,
        root: &mut eventually::aggregate::Root<Activity>,
    ) -> Result<(), SaveError> {
        self.repository.save(root).await
    }
}

impl ActivityRepositoryTrait for ActivityRepository {}
