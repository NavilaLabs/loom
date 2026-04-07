pub(crate) mod application;
pub(crate) mod domain;

pub use application::{
    Error as ApplicationError, commands::WorkspaceRoleCommand, views::WorkspaceRoleView,
};
pub use domain::{
    Error as DomainError,
    aggregates::{WorkspaceRole, WorkspaceRoleId},
    events::WorkspaceRoleEvent,
    interfaces::WorkspaceRoleRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    ApplicationError(#[from] application::Error),
    #[error("{0:?}")]
    DomainError(#[from] domain::Error),
}
