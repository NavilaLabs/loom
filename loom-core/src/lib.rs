pub mod admin;
pub mod permissions;
pub mod shared;
pub mod tenant;
pub mod validation;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    AdminDatabaseError(#[from] admin::Error),
    #[error("{0:?}")]
    TenantDatabaseError(#[from] tenant::Error),

    #[error("{0:?}")]
    ParseUuidError(#[from] uuid::Error),
    #[error("{0:?}")]
    SerdeJsonError(#[from] serde_json::Error),
}
