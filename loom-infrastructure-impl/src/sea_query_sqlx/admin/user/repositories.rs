use std::{ops::Deref, str::FromStr};

use async_trait::async_trait;
use eventually::serde::Json;
use eventually_any::snapshot::Repository;
use loom_core::admin::{
    Query, RowToView,
    user::{User, UserEvent, UserView},
};
use sea_orm::ExprTrait;
use sea_query::Expr;
use sqlx::{Row, any::AnyRow, types::Uuid};

use crate::ConnectedAdminPool;

pub struct UserRepository {
    database: ConnectedAdminPool,
    repository: Repository<User, Json<User>, Json<UserEvent>>,
}

impl Deref for UserRepository {
    type Target = Repository<User, Json<User>, Json<UserEvent>>;

    fn deref(&self) -> &Self::Target {
        &self.repository
    }
}

impl RowToView<AnyRow, UserView> for UserRepository {
    type Error = crate::Error;
    type View = UserView;

    fn row_to_view(&self, row: AnyRow) -> Result<UserView, Self::Error> {
        let id: String = row.try_get("id")?;
        let id = Uuid::from_str(&id)?;
        let name: String = row.try_get("name")?;

        Ok(UserView::new(id.into(), name))
    }
}

#[async_trait]
impl Query<AnyRow, UserView> for UserRepository {
    const Table: &'static str = "projections__users";

    async fn one(&self, id: Uuid) -> Result<Self::View, Self::Error> {
        let statement = sea_query::Query::select()
            .from(Self::Table)
            .and_where(Expr::Column("id".into()).eq(id))
            .to_owned();
        let (sql, arguments) = self.database.get_database_type().build_query(&statement);

        let row = sqlx::query_with(&sql, arguments)
            .fetch_one(self.database.as_ref())
            .await?;

        self.row_to_view(row)
    }

    async fn all(&self) -> Result<Self::View, Self::Error> {
        todo!()
    }
}
