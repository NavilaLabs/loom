pub(crate) mod application;
pub(crate) mod domain;

pub use application::{commands::ProjectRateCommand, views::ProjectRateView};
pub use domain::{
    Error as DomainError,
    aggregates::{ProjectRate, ProjectRateId},
    events::ProjectRateEvent,
    interfaces::ProjectRateRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    DomainError(#[from] domain::Error),
}
