pub mod tenant;
pub mod user;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    UserError(#[from] user::aggregates::Error),
}

impl From<user::aggregates::Error> for crate::Error {
    fn from(value: user::aggregates::Error) -> Self {
        crate::Error::AdminDatabaseError(Error::UserError(value))
    }
}
