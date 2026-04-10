pub mod authenticator;
pub mod permission;
pub mod user;
pub mod workspace;
pub mod workspace_role;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    UserError(#[from] user::Error),
    #[error("{0:?}")]
    PermissionError(#[from] permission::Error),
    #[error("{0:?}")]
    WorkspaceError(#[from] workspace::Error),
    #[error("{0:?}")]
    WorkspaceRoleError(#[from] workspace_role::Error),
}

impl From<user::ApplicationError> for crate::Error {
    fn from(value: user::ApplicationError) -> Self {
        Self::AdminDatabaseError(Error::UserError(value.into()))
    }
}

impl From<user::DomainError> for crate::Error {
    fn from(value: user::DomainError) -> Self {
        Self::AdminDatabaseError(Error::UserError(value.into()))
    }
}

impl From<permission::DomainError> for crate::Error {
    fn from(value: permission::DomainError) -> Self {
        Self::AdminDatabaseError(Error::PermissionError(value.into()))
    }
}

impl From<workspace::DomainError> for crate::Error {
    fn from(value: workspace::DomainError) -> Self {
        Self::AdminDatabaseError(Error::WorkspaceError(value.into()))
    }
}

impl From<workspace_role::DomainError> for crate::Error {
    fn from(value: workspace_role::DomainError) -> Self {
        Self::AdminDatabaseError(Error::WorkspaceRoleError(value.into()))
    }
}
