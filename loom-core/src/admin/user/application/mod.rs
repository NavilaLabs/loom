pub mod commands;
pub mod queries;
pub mod views;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("user not found")]
    UserNotFound,
    #[error("authentication failed: {0}")]
    AuthenticationFailed(String),
    #[error("repository error: {0}")]
    RepositoryError(String),
}
