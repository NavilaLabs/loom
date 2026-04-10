use std::ops::Deref;

use async_trait::async_trait;
use eventually::aggregate::repository::{GetError, Getter, SaveError, Saver};
use eventually::serde::Json;
use eventually_any::snapshot::Repository;
use loom_core::tenant::customer::{
    Customer, CustomerEvent, CustomerId, CustomerRepository as CustomerRepositoryTrait,
};
use sqlx::{Row, any::AnyRow};

use crate::ConnectedTenantPool;

pub struct CustomerRepository {
    pool: ConnectedTenantPool,
    repository: Repository<Customer, Json<Customer>, Json<CustomerEvent>>,
}

impl Deref for CustomerRepository {
    type Target = Repository<Customer, Json<Customer>, Json<CustomerEvent>>;
    fn deref(&self) -> &Self::Target {
        &self.repository
    }
}

impl CustomerRepository {
    pub async fn from_pool(pool: ConnectedTenantPool) -> Result<Self, sqlx::migrate::MigrateError> {
        let repository =
            Repository::new(pool.as_ref().clone(), Json::default(), Json::default()).await?;
        Ok(Self { pool, repository })
    }

    pub async fn all(&self) -> Result<Vec<CustomerRow>, crate::Error> {
        let rows = sqlx::query(
            "SELECT id, name, comment, currency, timezone, country, visible, \
             time_budget, money_budget, budget_is_monthly \
             FROM projections__customers ORDER BY name",
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        rows.into_iter().map(|r| Self::map_row(&r)).collect()
    }

    fn map_row(row: &AnyRow) -> Result<CustomerRow, crate::Error> {
        Ok(CustomerRow {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            comment: row.try_get("comment")?,
            currency: row.try_get("currency")?,
            timezone: row.try_get("timezone")?,
            country: row.try_get("country")?,
            visible: bool_col(row, "visible"),
            time_budget: row.try_get("time_budget")?,
            money_budget: row.try_get("money_budget")?,
            budget_is_monthly: bool_col(row, "budget_is_monthly"),
        })
    }
}

/// SQLite stores BOOLEAN as INTEGER; the `any` driver may decode it as i64.
fn bool_col(row: &AnyRow, col: &str) -> bool {
    row.try_get::<bool, _>(col)
        .unwrap_or_else(|_| row.try_get::<i64, _>(col).map(|v| v != 0).unwrap_or(false))
}

#[derive(Debug, Clone)]
pub struct CustomerRow {
    pub id: String,
    pub name: String,
    pub comment: Option<String>,
    pub currency: String,
    pub timezone: String,
    pub country: Option<String>,
    pub visible: bool,
    pub time_budget: Option<i32>,
    pub money_budget: Option<i64>,
    pub budget_is_monthly: bool,
}

#[async_trait]
impl Getter<Customer> for CustomerRepository {
    async fn get(
        &self,
        id: &CustomerId,
    ) -> Result<eventually::aggregate::Root<Customer>, GetError> {
        self.repository.get(id).await
    }
}

#[async_trait]
impl Saver<Customer> for CustomerRepository {
    async fn save(
        &self,
        root: &mut eventually::aggregate::Root<Customer>,
    ) -> Result<(), SaveError> {
        self.repository.save(root).await
    }
}

impl CustomerRepositoryTrait for CustomerRepository {}
