use std::ops::Deref;

use async_trait::async_trait;
use eventually::aggregate::repository::{GetError, Getter, SaveError, Saver};
use eventually::serde::Json;
use eventually_any::snapshot::Repository;
use loom_core::tenant::project_rate::{
    ProjectRate, ProjectRateEvent, ProjectRateId, ProjectRateRepository as ProjectRateRepositoryTrait,
};
use sqlx::{Row, any::AnyRow};

use crate::ConnectedTenantPool;

pub struct ProjectRateRepository {
    pool: ConnectedTenantPool,
    repository: Repository<ProjectRate, Json<ProjectRate>, Json<ProjectRateEvent>>,
}

impl Deref for ProjectRateRepository {
    type Target = Repository<ProjectRate, Json<ProjectRate>, Json<ProjectRateEvent>>;
    fn deref(&self) -> &Self::Target { &self.repository }
}

impl ProjectRateRepository {
    pub async fn from_pool(pool: ConnectedTenantPool) -> Result<Self, sqlx::migrate::MigrateError> {
        let repository =
            Repository::new(pool.as_ref().clone(), Json::default(), Json::default()).await?;
        Ok(Self { pool, repository })
    }

    pub async fn for_project(&self, project_id: &str) -> Result<Vec<ProjectRateRow>, crate::Error> {
        let rows = sqlx::query(
            "SELECT id, project_id, user_id, hourly_rate, internal_rate \
             FROM projections__project_rates \
             WHERE project_id = ? \
             ORDER BY user_id NULLS FIRST",
        )
        .bind(project_id)
        .fetch_all(self.pool.as_ref())
        .await?;
        rows.into_iter().map(|r| Self::map_row(&r)).collect()
    }

    pub async fn default_for_project(
        &self,
        project_id: &str,
    ) -> Result<Option<ProjectRateRow>, crate::Error> {
        let row = sqlx::query(
            "SELECT id, project_id, user_id, hourly_rate, internal_rate \
             FROM projections__project_rates \
             WHERE project_id = ? AND user_id IS NULL \
             LIMIT 1",
        )
        .bind(project_id)
        .fetch_optional(self.pool.as_ref())
        .await?;
        row.as_ref().map(Self::map_row).transpose()
    }

    fn map_row(row: &AnyRow) -> Result<ProjectRateRow, crate::Error> {
        Ok(ProjectRateRow {
            id: row.try_get("id")?,
            project_id: row.try_get("project_id")?,
            user_id: row.try_get("user_id")?,
            hourly_rate: row.try_get::<i64, _>("hourly_rate")?,
            internal_rate: row.try_get("internal_rate")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ProjectRateRow {
    pub id: String,
    pub project_id: String,
    pub user_id: Option<String>,
    pub hourly_rate: i64,
    pub internal_rate: Option<i64>,
}

#[async_trait]
impl Getter<ProjectRate> for ProjectRateRepository {
    async fn get(
        &self,
        id: &ProjectRateId,
    ) -> Result<eventually::aggregate::Root<ProjectRate>, GetError> {
        self.repository.get(id).await
    }
}

#[async_trait]
impl Saver<ProjectRate> for ProjectRateRepository {
    async fn save(
        &self,
        root: &mut eventually::aggregate::Root<ProjectRate>,
    ) -> Result<(), SaveError> {
        self.repository.save(root).await
    }
}

impl ProjectRateRepositoryTrait for ProjectRateRepository {}
