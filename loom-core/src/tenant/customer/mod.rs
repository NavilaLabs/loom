pub(crate) mod application;
pub(crate) mod domain;

pub use application::{commands::CustomerCommand, views::CustomerView};
pub use domain::{
    Error as DomainError,
    aggregates::{Customer, CustomerId},
    events::CustomerEvent,
    interfaces::CustomerRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    DomainError(#[from] domain::Error),
}
