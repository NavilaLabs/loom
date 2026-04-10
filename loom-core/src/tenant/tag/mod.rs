pub(crate) mod application;
pub(crate) mod domain;

pub use application::{
    commands::TagCommand,
    inputs::{CreateTagInput, RenameTagInput},
    views::TagView,
};
pub use domain::{
    Error as DomainError,
    aggregates::{Tag, TagId},
    events::TagEvent,
    interfaces::TagRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    DomainError(#[from] domain::Error),
}
