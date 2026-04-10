use async_trait::async_trait;
use eventually_projection::{Projector, RawEvent};
use loom_core::tenant::activity_rate::ActivityRateEvent;
use sea_query::{Condition, DynIden, Expr, ExprTrait, OnConflict, Query, TableRef};
use sea_query_sqlx::SqlxBinder;

use crate::{ConnectedTenantPool, DatabaseType};

pub struct ActivityRateProjector {
    pool: ConnectedTenantPool,
}

impl ActivityRateProjector {
    const TABLE: &'static str = "projections__activity_rates";

    #[must_use] 
    pub const fn new(pool: ConnectedTenantPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Projector for ActivityRateProjector {
    type Error = crate::Error;

    async fn handle(&mut self, event: RawEvent) -> Result<(), Self::Error> {
        match event.event_type.as_str() {
            "ActivityRateSet" => {
                let ActivityRateEvent::Set {
                    id,
                    activity_id,
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
                        DynIden::from("activity_id"),
                        DynIden::from("user_id"),
                        DynIden::from("hourly_rate"),
                        DynIden::from("internal_rate"),
                    ])
                    .values_panic([
                        id.to_string().into(),
                        activity_id.to_string().into(),
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
            "ActivityRateRemoved" => {
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
