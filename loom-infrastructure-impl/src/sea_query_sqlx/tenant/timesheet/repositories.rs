use std::ops::Deref;

use async_trait::async_trait;
use eventually::aggregate::repository::{GetError, Getter, SaveError, Saver};
use eventually::serde::Json;
use eventually_any::snapshot::Repository;
use loom_core::tenant::timesheet::{Timesheet, TimesheetEvent, TimesheetId, TimesheetRepository as TimesheetRepositoryTrait};
use sqlx::{Row, any::AnyRow};

use crate::ConnectedTenantPool;

pub struct TimesheetRepository {
    pool: ConnectedTenantPool,
    repository: Repository<Timesheet, Json<Timesheet>, Json<TimesheetEvent>>,
}

impl Deref for TimesheetRepository {
    type Target = Repository<Timesheet, Json<Timesheet>, Json<TimesheetEvent>>;
    fn deref(&self) -> &Self::Target { &self.repository }
}

impl TimesheetRepository {
    pub async fn from_pool(pool: ConnectedTenantPool) -> Result<Self, sqlx::migrate::MigrateError> {
        let repository =
            Repository::new(pool.as_ref().clone(), Json::default(), Json::default()).await?;
        Ok(Self { pool, repository })
    }

    /// Most-recent 50 timesheets for a user, newest first.
    pub async fn recent_for_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<TimesheetRow>, crate::Error> {
        let rows = sqlx::query(
            "SELECT id, user_id, project_id, activity_id, start_time, end_time, \
             duration, description, timezone, billable, exported \
             FROM projections__timesheets WHERE user_id = ? \
             ORDER BY start_time DESC LIMIT 50",
        )
        .bind(user_id)
        .fetch_all(self.pool.as_ref())
        .await?;
        rows.into_iter().map(|r| Self::map_row(&r)).collect()
    }

    /// Returns the running timesheet for a user (end_time IS NULL), if any.
    pub async fn running_for_user(
        &self,
        user_id: &str,
    ) -> Result<Option<TimesheetRow>, crate::Error> {
        let row = sqlx::query(
            "SELECT id, user_id, project_id, activity_id, start_time, end_time, \
             duration, description, timezone, billable, exported \
             FROM projections__timesheets WHERE user_id = ? AND end_time IS NULL \
             ORDER BY start_time DESC LIMIT 1",
        )
        .bind(user_id)
        .fetch_optional(self.pool.as_ref())
        .await?;
        row.map(|r| Self::map_row(&r)).transpose()
    }

    fn map_row(row: &AnyRow) -> Result<TimesheetRow, crate::Error> {
        Ok(TimesheetRow {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            project_id: row.try_get("project_id")?,
            activity_id: row.try_get("activity_id")?,
            start_time: row.try_get("start_time")?,
            end_time: row.try_get("end_time")?,
            duration: row.try_get("duration")?,
            description: row.try_get("description")?,
            timezone: row.try_get("timezone")?,
            billable: bool_col(row, "billable"),
            exported: bool_col(row, "exported"),
        })
    }
}

fn bool_col(row: &AnyRow, col: &str) -> bool {
    row.try_get::<bool, _>(col)
        .unwrap_or_else(|_| row.try_get::<i64, _>(col).map(|v| v != 0).unwrap_or(false))
}

#[derive(Debug, Clone)]
pub struct TimesheetRow {
    pub id: String,
    pub user_id: String,
    pub project_id: String,
    pub activity_id: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration: Option<i32>,
    pub description: Option<String>,
    pub timezone: String,
    pub billable: bool,
    pub exported: bool,
}

#[async_trait]
impl Getter<Timesheet> for TimesheetRepository {
    async fn get(
        &self,
        id: &TimesheetId,
    ) -> Result<eventually::aggregate::Root<Timesheet>, GetError> {
        self.repository.get(id).await
    }
}

#[async_trait]
impl Saver<Timesheet> for TimesheetRepository {
    async fn save(
        &self,
        root: &mut eventually::aggregate::Root<Timesheet>,
    ) -> Result<(), SaveError> {
        self.repository.save(root).await
    }
}

impl TimesheetRepositoryTrait for TimesheetRepository {}
