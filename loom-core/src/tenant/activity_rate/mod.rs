pub(crate) mod application;
pub(crate) mod domain;

pub use application::{commands::ActivityRateCommand, views::ActivityRateView};
pub use domain::{
    Error as DomainError,
    aggregates::{ActivityRate, ActivityRateId},
    events::ActivityRateEvent,
    interfaces::ActivityRateRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    DomainError(#[from] domain::Error),
}
