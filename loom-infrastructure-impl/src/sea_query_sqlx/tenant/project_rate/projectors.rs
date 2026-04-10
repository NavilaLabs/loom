use async_trait::async_trait;
use eventually_projection::{Projector, RawEvent};
use loom_core::tenant::project_rate::ProjectRateEvent;
use sea_query::{Condition, DynIden, Expr, ExprTrait, OnConflict, Query, TableRef};
use sea_query_sqlx::SqlxBinder;

use crate::{ConnectedTenantPool, DatabaseType};

pub struct ProjectRateProjector {
    pool: ConnectedTenantPool,
}

impl ProjectRateProjector {
    const TABLE: &'static str = "projections__project_rates";

    pub fn new(pool: ConnectedTenantPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Projector for ProjectRateProjector {
    type Error = crate::Error;

    async fn handle(&mut self, event: RawEvent) -> Result<(), Self::Error> {
        match event.event_type.as_str() {
            "ProjectRateSet" => {
                let ProjectRateEvent::Set {
                    id,
                    project_id,
                    user_id,
                    hourly_rate,
                    internal_rate,
                } = serde_json::from_slice(&event.payload_bytes)?
                else {
                    return Ok(());
                };

                let query = Query::insert()
                    .into_table(TableRef::from(Self::TABLE))
                    .columns([
                        DynIden::from("id"),
                        DynIden::from("project_id"),
                        DynIden::from("user_id"),
                        DynIden::from("hourly_rate"),
                        DynIden::from("internal_rate"),
                    ])
                    .values_panic([
                        id.to_string().into(),
                        project_id.to_string().into(),
                        user_id.map(|u| u.to_string()).into(),
                        hourly_rate.into(),
                        internal_rate.into(),
                    ])
                    .on_conflict(OnConflict::new().do_nothing().to_owned())
                    .to_owned();

                let (sql, values) = self.pool.build_query(&query);
                sqlx::query_with(&sql, values)
                    .execute(self.pool.as_ref())
                    .await?;
            }
            "ProjectRateRemoved" => {
                let query = Query::delete()
                    .from_table(TableRef::from(Self::TABLE))
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
