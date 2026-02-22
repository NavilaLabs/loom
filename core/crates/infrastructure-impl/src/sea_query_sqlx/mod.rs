pub mod infrastructure;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    InfrastructureError(#[from] infrastructure::Error),
}
