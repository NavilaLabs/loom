use async_trait::async_trait;
use eventually_projection::{Projector, RawEvent};
use loom_core::tenant::timesheet::TimesheetEvent;
use sea_query::{Condition, DynIden, Expr, ExprTrait, OnConflict, Query, TableRef};
use sea_query_sqlx::SqlxBinder;

use crate::{ConnectedTenantPool, DatabaseType};

pub struct TimesheetProjector {
    pool: ConnectedTenantPool,
}

impl TimesheetProjector {
    const TABLE: &'static str = "projections__timesheets";

    pub fn new(pool: ConnectedTenantPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Projector for TimesheetProjector {
    type Error = crate::Error;

    async fn handle(&mut self, event: RawEvent) -> Result<(), Self::Error> {
        match event.event_type.as_str() {
            "TimesheetStarted" => {
                let TimesheetEvent::Started {
                    id,
                    user_id,
                    project_id,
                    activity_id,
                    start_time,
                    timezone,
                    billable,
                } = serde_json::from_slice(&event.payload_bytes)?
                else {
                    return Ok(());
                };

                let query = Query::insert()
                    .into_table(TableRef::from(Self::TABLE))
                    .columns([
                        DynIden::from("id"),
                        DynIden::from("user_id"),
                        DynIden::from("project_id"),
                        DynIden::from("activity_id"),
                        DynIden::from("start_time"),
                        DynIden::from("timezone"),
                        DynIden::from("billable"),
                    ])
                    .values_panic([
                        id.to_string().into(),
                        user_id.to_string().into(),
                        project_id.map(|v| v.to_string()).into(),
                        activity_id.map(|v| v.to_string()).into(),
                        start_time.into(),
                        timezone.into(),
                        billable.into(),
                    ])
                    .on_conflict(OnConflict::new().do_nothing().to_owned())
                    .to_owned();

                let (sql, values) = self.pool.build_query(&query);
                sqlx::query_with(&sql, values)
                    .execute(self.pool.as_ref())
                    .await?;
            }
            "TimesheetStopped" => {
                let TimesheetEvent::Stopped {
                    end_time,
                    duration,
                    hourly_rate,
                    fixed_rate,
                    internal_rate,
                    rate,
                } = serde_json::from_slice(&event.payload_bytes)?
                else {
                    return Ok(());
                };

                let query = Query::update()
                    .table(TableRef::from(Self::TABLE))
                    .values([
                        (DynIden::from("end_time"), end_time.into()),
                        (DynIden::from("duration"), duration.into()),
                        (DynIden::from("hourly_rate"), hourly_rate.into()),
                        (DynIden::from("fixed_rate"), fixed_rate.into()),
                        (DynIden::from("internal_rate"), internal_rate.into()),
                        (DynIden::from("rate"), rate.into()),
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
            "TimesheetUpdated" => {
                let TimesheetEvent::Updated {
                    description,
                    billable,
                } = serde_json::from_slice(&event.payload_bytes)?
                else {
                    return Ok(());
                };

                let query = Query::update()
                    .table(TableRef::from(Self::TABLE))
                    .values([
                        (DynIden::from("description"), description.into()),
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
            "TimesheetReassigned" => {
                let TimesheetEvent::Reassigned {
                    project_id,
                    activity_id,
                } = serde_json::from_slice(&event.payload_bytes)?
                else {
                    return Ok(());
                };

                let query = Query::update()
                    .table(TableRef::from(Self::TABLE))
                    .values([
                        (DynIden::from("project_id"), project_id.to_string().into()),
                        (DynIden::from("activity_id"), activity_id.to_string().into()),
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
            "TimesheetExported" => {
                let query = Query::update()
                    .table(TableRef::from(Self::TABLE))
                    .values([(DynIden::from("exported"), true.into())])
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
