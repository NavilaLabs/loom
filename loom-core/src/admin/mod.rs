use std::fmt::Debug;

use async_trait::async_trait;
use uuid::Uuid;

pub mod authenticator;
pub mod user;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    UserError(#[from] user::Error),
}

impl From<user::ApplicationError> for crate::Error {
    fn from(value: user::ApplicationError) -> Self {
        crate::Error::AdminDatabaseError(Error::UserError(value.into()))
    }
}

impl From<user::DomainError> for crate::Error {
    fn from(value: user::DomainError) -> Self {
        crate::Error::AdminDatabaseError(Error::UserError(value.into()))
    }
}

pub trait RowToView<R, V> {
    type Error: Debug;
    type View;

    fn row_to_view(&self, row: R) -> Result<V, Self::Error>;
}

#[async_trait]
pub trait Query<R, V>: RowToView<R, V> {
    const Table: &'static str;

    async fn one(&self, id: Uuid) -> Result<Self::View, Self::Error>;

    async fn all(&self) -> Result<Self::View, Self::Error>;
}
