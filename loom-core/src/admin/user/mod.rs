mod application;
mod domain;
mod infrastructure;

pub use application::*;
pub use domain::*;
pub use infrastructure::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "eventually")]
    #[error("{0}")]
    SaveError(#[from] eventually::aggregate::repository::SaveError),
}
