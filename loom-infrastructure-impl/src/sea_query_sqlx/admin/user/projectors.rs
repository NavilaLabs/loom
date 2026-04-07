use async_trait::async_trait;
use eventually_projection::{Projector, RawEvent};
use loom_core::admin::user::UserEvent;
use sea_query::{DynIden, PostgresQueryBuilder, Query, SqliteQueryBuilder, TableRef};
use sea_query_sqlx::SqlxBinder;

use crate::{DatabaseType, Pool, ScopeAdmin, StateConnected};

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
        match event.event_type.as_str() {
            "UserCreated" => {
                let UserEvent::Created {
                    id,
                    name,
                    email,
                    password_hash,
                } = serde_json::from_slice(&event.payload_bytes)?;

                let query = Query::insert()
                    .into_table(TableRef::from(Self::TABLE))
                    .columns([
                        DynIden::from("id"),
                        DynIden::from("name"),
                        DynIden::from("email"),
                        DynIden::from("password"),
                        DynIden::from("salt"),
                    ])
                    .values_panic([
                        id.to_string().into(),
                        name.into(),
                        email.into(),
                        password_hash.into(),
                        "".into(),
                    ])
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
