pub(crate) mod application;
pub(crate) mod domain;

pub use application::{Error as ApplicationError, commands::PermissionCommand, views::PermissionView};
pub use domain::{
    Error as DomainError,
    aggregates::{Permission, PermissionId},
    events::PermissionEvent,
    interfaces::PermissionRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    ApplicationError(#[from] application::Error),
    #[error("{0:?}")]
    DomainError(#[from] domain::Error),
}
