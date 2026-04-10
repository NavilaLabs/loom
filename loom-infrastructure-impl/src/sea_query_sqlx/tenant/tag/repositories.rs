use std::ops::Deref;

use async_trait::async_trait;
use eventually::aggregate::repository::{GetError, Getter, SaveError, Saver};
use eventually::serde::Json;
use eventually_any::snapshot::Repository;
use loom_core::tenant::tag::{Tag, TagEvent, TagId, TagRepository as TagRepositoryTrait};
use sqlx::{Row, any::AnyRow};

use crate::ConnectedTenantPool;

pub struct TagRepository {
    pool: ConnectedTenantPool,
    repository: Repository<Tag, Json<Tag>, Json<TagEvent>>,
}

impl Deref for TagRepository {
    type Target = Repository<Tag, Json<Tag>, Json<TagEvent>>;
    fn deref(&self) -> &Self::Target {
        &self.repository
    }
}

impl TagRepository {
    pub async fn from_pool(pool: ConnectedTenantPool) -> Result<Self, sqlx::migrate::MigrateError> {
        let repository =
            Repository::new(pool.as_ref().clone(), Json::default(), Json::default()).await?;
        Ok(Self { pool, repository })
    }

    pub async fn all(&self) -> Result<Vec<TagRow>, crate::Error> {
        let rows = sqlx::query("SELECT id, name FROM projections__tags ORDER BY name")
            .fetch_all(self.pool.as_ref())
            .await?;
        rows.into_iter().map(|r| Self::map_row(&r)).collect()
    }

    pub async fn for_timesheet(&self, timesheet_id: &str) -> Result<Vec<TagRow>, crate::Error> {
        let rows = sqlx::query(
            "SELECT t.id, t.name \
             FROM projections__tags t \
             JOIN projections__timesheet_tags tt ON tt.tag_id = t.id \
             WHERE tt.timesheet_id = ? \
             ORDER BY t.name",
        )
        .bind(timesheet_id)
        .fetch_all(self.pool.as_ref())
        .await?;
        rows.into_iter().map(|r| Self::map_row(&r)).collect()
    }

    fn map_row(row: &AnyRow) -> Result<TagRow, crate::Error> {
        Ok(TagRow {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TagRow {
    pub id: String,
    pub name: String,
}

#[async_trait]
impl Getter<Tag> for TagRepository {
    async fn get(&self, id: &TagId) -> Result<eventually::aggregate::Root<Tag>, GetError> {
        self.repository.get(id).await
    }
}

#[async_trait]
impl Saver<Tag> for TagRepository {
    async fn save(&self, root: &mut eventually::aggregate::Root<Tag>) -> Result<(), SaveError> {
        self.repository.save(root).await
    }
}

impl TagRepositoryTrait for TagRepository {}
