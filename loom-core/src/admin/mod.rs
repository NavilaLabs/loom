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
