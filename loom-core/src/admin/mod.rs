pub mod tenant;
pub mod user;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
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

impl From<user::InfrastructureError> for crate::Error {
    fn from(value: user::InfrastructureError) -> Self {
        crate::Error::AdminDatabaseError(Error::UserError(value.into()))
    }
}
