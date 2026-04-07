pub(crate) mod application;
pub(crate) mod domain;

pub use application::{
    Error as ApplicationError, commands::WorkspaceCommand, views::WorkspaceView,
};
pub use domain::{
    Error as DomainError,
    aggregates::{Workspace, WorkspaceId},
    events::WorkspaceEvent,
    interfaces::WorkspaceRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    ApplicationError(#[from] application::Error),
    #[error("{0:?}")]
    DomainError(#[from] domain::Error),
}
