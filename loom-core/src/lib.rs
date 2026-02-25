pub mod tenant;
pub mod user;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    UuidError(#[from] uuid::Error),
    #[error("{0}")]
    HexError(#[from] hex::FromHexError),
}
