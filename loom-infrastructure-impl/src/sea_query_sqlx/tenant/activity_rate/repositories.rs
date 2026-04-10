use std::ops::Deref;

use async_trait::async_trait;
use eventually::aggregate::repository::{GetError, Getter, SaveError, Saver};
use eventually::serde::Json;
use eventually_any::snapshot::Repository;
use loom_core::tenant::activity_rate::{
    ActivityRate, ActivityRateEvent, ActivityRateId,
    ActivityRateRepository as ActivityRateRepositoryTrait,
};
use sqlx::{Row, any::AnyRow};

use crate::ConnectedTenantPool;

pub struct ActivityRateRepository {
    pool: ConnectedTenantPool,
    repository: Repository<ActivityRate, Json<ActivityRate>, Json<ActivityRateEvent>>,
}

impl Deref for ActivityRateRepository {
    type Target = Repository<ActivityRate, Json<ActivityRate>, Json<ActivityRateEvent>>;
    fn deref(&self) -> &Self::Target {
        &self.repository
    }
}

impl ActivityRateRepository {
    pub async fn from_pool(pool: ConnectedTenantPool) -> Result<Self, sqlx::migrate::MigrateError> {
        let repository =
            Repository::new(pool.as_ref().clone(), Json::default(), Json::default()).await?;
        Ok(Self { pool, repository })
    }

    pub async fn for_activity(
        &self,
        activity_id: &str,
    ) -> Result<Vec<ActivityRateRow>, crate::Error> {
        let rows = sqlx::query(
            "SELECT id, activity_id, user_id, hourly_rate, internal_rate \
             FROM projections__activity_rates \
             WHERE activity_id = ? \
             ORDER BY user_id NULLS FIRST",
        )
        .bind(activity_id)
        .fetch_all(self.pool.as_ref())
        .await?;
        rows.into_iter().map(|r| Self::map_row(&r)).collect()
    }

    pub async fn default_for_activity(
        &self,
        activity_id: &str,
    ) -> Result<Option<ActivityRateRow>, crate::Error> {
        let row = sqlx::query(
            "SELECT id, activity_id, user_id, hourly_rate, internal_rate \
             FROM projections__activity_rates \
             WHERE activity_id = ? AND user_id IS NULL \
             LIMIT 1",
        )
        .bind(activity_id)
        .fetch_optional(self.pool.as_ref())
        .await?;
        row.as_ref().map(Self::map_row).transpose()
    }

    fn map_row(row: &AnyRow) -> Result<ActivityRateRow, crate::Error> {
        Ok(ActivityRateRow {
            id: row.try_get("id")?,
            activity_id: row.try_get("activity_id")?,
            user_id: row.try_get("user_id")?,
            hourly_rate: row.try_get::<i64, _>("hourly_rate")?,
            internal_rate: row.try_get("internal_rate")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ActivityRateRow {
    pub id: String,
    pub activity_id: String,
    pub user_id: Option<String>,
    pub hourly_rate: i64,
    pub internal_rate: Option<i64>,
}

#[async_trait]
impl Getter<ActivityRate> for ActivityRateRepository {
    async fn get(
        &self,
        id: &ActivityRateId,
    ) -> Result<eventually::aggregate::Root<ActivityRate>, GetError> {
        self.repository.get(id).await
    }
}

#[async_trait]
impl Saver<ActivityRate> for ActivityRateRepository {
    async fn save(
        &self,
        root: &mut eventually::aggregate::Root<ActivityRate>,
    ) -> Result<(), SaveError> {
        self.repository.save(root).await
    }
}

impl ActivityRateRepositoryTrait for ActivityRateRepository {}
