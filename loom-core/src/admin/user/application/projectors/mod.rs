use async_trait::async_trait;
use eventually_projection::{Projector, RawEvent};
use loom_infrastructure_impl::{DatabaseType, Pool, ScopeAdmin, StateConnected};
use sea_query::{DynIden, PostgresQueryBuilder, Query, SqliteQueryBuilder, TableRef};
use sea_query_sqlx::SqlxBinder;

use crate::admin::user::events::UserEvent;

pub struct UserProjector {
    pool: Pool<ScopeAdmin, StateConnected>,
}

impl UserProjector {
    const TABLE: &'static str = "projections__users";

    pub fn new(pool: Pool<ScopeAdmin, StateConnected>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Projector for UserProjector {
    type Error = crate::Error;

    async fn handle(&mut self, event: RawEvent) -> Result<(), Self::Error> {
        let UserEvent::Created { id, name } = serde_json::from_slice(&event.payload_bytes)?;

        match event.event_type.as_str() {
            "UserCreated" => {
                let query = Query::insert()
                    .into_table(TableRef::from(Self::TABLE))
                    .columns([DynIden::from("id"), DynIden::from("name")])
                    .values_panic([id.to_string().into(), name.into()])
                    .to_owned();
                let (sql, values) = match self.pool.get_database_type() {
                    DatabaseType::Sqlite => query.build_sqlx(SqliteQueryBuilder),
                    DatabaseType::Postgres => query.build_sqlx(PostgresQueryBuilder),
                };
                sqlx::query_with(&sql, values)
                    .execute(self.pool.as_ref())
                    .await?;

                Ok(())
            }
            _ => Ok(()),
        }
    }
}
