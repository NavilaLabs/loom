pub mod admin;
pub(crate) mod shared;
pub mod tenant;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    UuidError(#[from] uuid::Error),
}
