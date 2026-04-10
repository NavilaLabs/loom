use async_trait::async_trait;
use eventually_projection::{Projector, RawEvent};
use loom_core::tenant::project::ProjectEvent;
use sea_query::{Condition, DynIden, Expr, ExprTrait, OnConflict, Query, TableRef};
use sea_query_sqlx::SqlxBinder;

use crate::{ConnectedTenantPool, DatabaseType};

pub struct ProjectProjector {
    pool: ConnectedTenantPool,
}

impl ProjectProjector {
    const TABLE: &'static str = "projections__projects";

    pub fn new(pool: ConnectedTenantPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Projector for ProjectProjector {
    type Error = crate::Error;

    async fn handle(&mut self, event: RawEvent) -> Result<(), Self::Error> {
        match event.event_type.as_str() {
            "ProjectCreated" => {
                let ProjectEvent::Created {
                    id,
                    customer_id,
                    name,
                } = serde_json::from_slice(&event.payload_bytes)?
                else {
                    return Ok(());
                };

                let query = Query::insert()
                    .into_table(TableRef::from(Self::TABLE))
                    .columns([
                        DynIden::from("id"),
                        DynIden::from("customer_id"),
                        DynIden::from("name"),
                    ])
                    .values_panic([
                        id.to_string().into(),
                        customer_id.to_string().into(),
                        name.into(),
                    ])
                    .on_conflict(OnConflict::new().do_nothing().to_owned())
                    .to_owned();

                let (sql, values) = self.pool.build_query(&query);
                sqlx::query_with(&sql, values)
                    .execute(self.pool.as_ref())
                    .await?;
            }
            "ProjectUpdated" => {
                let ProjectEvent::Updated {
                    name,
                    comment,
                    order_number,
                    visible,
                    billable,
                } = serde_json::from_slice(&event.payload_bytes)?
                else {
                    return Ok(());
                };

                let query = Query::update()
                    .table(TableRef::from(Self::TABLE))
                    .values([
                        (DynIden::from("name"), name.into()),
                        (DynIden::from("comment"), comment.into()),
                        (DynIden::from("order_number"), order_number.into()),
                        (DynIden::from("visible"), visible.into()),
                        (DynIden::from("billable"), billable.into()),
                    ])
                    .cond_where(
                        Condition::all()
                            .add(Expr::col("id").eq(Expr::val(event.stream_id.clone()))),
                    )
                    .to_owned();

                let (sql, values) = match self.pool.get_database_type() {
                    DatabaseType::Sqlite => query.build_sqlx(sea_query::SqliteQueryBuilder),
                    DatabaseType::Postgres => query.build_sqlx(sea_query::PostgresQueryBuilder),
                };
                sqlx::query_with(&sql, values)
                    .execute(self.pool.as_ref())
                    .await?;
            }
            "ProjectBudgetUpdated" => {
                let ProjectEvent::BudgetUpdated {
                    time_budget,
                    money_budget,
                    budget_is_monthly,
                } = serde_json::from_slice(&event.payload_bytes)?
                else {
                    return Ok(());
                };

                let query = Query::update()
                    .table(TableRef::from(Self::TABLE))
                    .values([
                        (DynIden::from("time_budget"), time_budget.into()),
                        (DynIden::from("money_budget"), money_budget.into()),
                        (DynIden::from("budget_is_monthly"), budget_is_monthly.into()),
                    ])
                    .cond_where(
                        Condition::all()
                            .add(Expr::col("id").eq(Expr::val(event.stream_id.clone()))),
                    )
                    .to_owned();

                let (sql, values) = match self.pool.get_database_type() {
                    DatabaseType::Sqlite => query.build_sqlx(sea_query::SqliteQueryBuilder),
                    DatabaseType::Postgres => query.build_sqlx(sea_query::PostgresQueryBuilder),
                };
                sqlx::query_with(&sql, values)
                    .execute(self.pool.as_ref())
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
