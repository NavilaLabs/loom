pub(crate) mod application;
pub(crate) mod domain;
pub(crate) mod infrastructure;

pub use application::{
    Error as ApplicationError, commands::UserRoot, use_cases::create_user, views::UserView,
};
pub use domain::{
    Error as DomainError,
    aggregates::{User, UserId},
    events::UserEvent,
    interfaces::UserRepository,
};
pub use infrastructure::{
    Error as InfrastructureError, projectors::UserProjector,
    repositories::UserRepository as UserRepositoryImpl,
};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    ApplicationError(#[from] application::Error),
    #[error("{0:?}")]
    DomainError(#[from] domain::Error),
    #[error("{0:?}")]
    InfrastructureError(#[from] infrastructure::Error),
}
