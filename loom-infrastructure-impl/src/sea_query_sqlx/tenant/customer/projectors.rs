use async_trait::async_trait;
use eventually_projection::{Projector, RawEvent};
use loom_core::tenant::customer::CustomerEvent;
use sea_query::{
    Condition, DynIden, Expr, ExprTrait, OnConflict, Query, TableRef,
};
use sea_query_sqlx::SqlxBinder;

use crate::{ConnectedTenantPool, DatabaseType};

pub struct CustomerProjector {
    pool: ConnectedTenantPool,
}

impl CustomerProjector {
    const TABLE: &'static str = "projections__customers";

    pub fn new(pool: ConnectedTenantPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Projector for CustomerProjector {
    type Error = crate::Error;

    async fn handle(&mut self, event: RawEvent) -> Result<(), Self::Error> {
        match event.event_type.as_str() {
            "CustomerCreated" => {
                let CustomerEvent::Created { id, name, currency, timezone } =
                    serde_json::from_slice(&event.payload_bytes)?
                else {
                    return Ok(());
                };

                let query = Query::insert()
                    .into_table(TableRef::from(Self::TABLE))
                    .columns([
                        DynIden::from("id"),
                        DynIden::from("name"),
                        DynIden::from("currency"),
                        DynIden::from("timezone"),
                    ])
                    .values_panic([
                        id.to_string().into(),
                        name.into(),
                        currency.into(),
                        timezone.into(),
                    ])
                    .on_conflict(OnConflict::new().do_nothing().to_owned())
                    .to_owned();

                let (sql, values) = self.pool.build_query(&query);
                sqlx::query_with(&sql, values).execute(self.pool.as_ref()).await?;
            }
            "CustomerUpdated" => {
                let CustomerEvent::Updated { name, comment, currency, timezone, country, visible } =
                    serde_json::from_slice(&event.payload_bytes)?
                else {
                    return Ok(());
                };

                let query = Query::update()
                    .table(TableRef::from(Self::TABLE))
                    .values([
                        (DynIden::from("name"), name.into()),
                        (DynIden::from("comment"), comment.into()),
                        (DynIden::from("currency"), currency.into()),
                        (DynIden::from("timezone"), timezone.into()),
                        (DynIden::from("country"), country.into()),
                        (DynIden::from("visible"), visible.into()),
                    ])
                    .cond_where(
                        Condition::all()
                            .add(Expr::col("id").eq(Expr::val(event.stream_id.clone()))),
                    )
                    .to_owned();

                let (sql, values) = match self.pool.get_database_type() {
                    DatabaseType::Sqlite => {
                        use sea_query::SqliteQueryBuilder;
                        query.build_sqlx(SqliteQueryBuilder)
                    }
                    DatabaseType::Postgres => {
                        use sea_query::PostgresQueryBuilder;
                        query.build_sqlx(PostgresQueryBuilder)
                    }
                };
                sqlx::query_with(&sql, values).execute(self.pool.as_ref()).await?;
            }
            "CustomerBudgetUpdated" => {
                let CustomerEvent::BudgetUpdated { time_budget, money_budget, budget_is_monthly } =
                    serde_json::from_slice(&event.payload_bytes)?
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
                    DatabaseType::Sqlite => {
                        use sea_query::SqliteQueryBuilder;
                        query.build_sqlx(SqliteQueryBuilder)
                    }
                    DatabaseType::Postgres => {
                        use sea_query::PostgresQueryBuilder;
                        query.build_sqlx(PostgresQueryBuilder)
                    }
                };
                sqlx::query_with(&sql, values).execute(self.pool.as_ref()).await?;
            }
            _ => {}
        }

        Ok(())
    }
}
