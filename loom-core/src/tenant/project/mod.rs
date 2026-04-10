pub(crate) mod application;
pub(crate) mod domain;

pub use application::{
    commands::ProjectCommand,
    inputs::{CreateProjectInput, UpdateProjectInput},
    views::ProjectView,
};
pub use domain::{
    Error as DomainError,
    aggregates::{Project, ProjectId},
    events::ProjectEvent,
    interfaces::ProjectRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    DomainError(#[from] domain::Error),
}
