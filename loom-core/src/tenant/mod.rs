pub mod activity;
pub mod activity_rate;
pub mod customer;
pub mod project;
pub mod project_rate;
pub mod tag;
pub mod timesheet;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    ActivityError(#[from] activity::Error),
    #[error("{0:?}")]
    ActivityRateError(#[from] activity_rate::Error),
    #[error("{0:?}")]
    CustomerError(#[from] customer::Error),
    #[error("{0:?}")]
    ProjectError(#[from] project::Error),
    #[error("{0:?}")]
    ProjectRateError(#[from] project_rate::Error),
    #[error("{0:?}")]
    TagError(#[from] tag::Error),
    #[error("{0:?}")]
    TimesheetError(#[from] timesheet::Error),
}

impl From<activity::DomainError> for crate::Error {
    fn from(value: activity::DomainError) -> Self {
        Self::TenantDatabaseError(Error::ActivityError(value.into()))
    }
}

impl From<activity_rate::DomainError> for crate::Error {
    fn from(value: activity_rate::DomainError) -> Self {
        Self::TenantDatabaseError(Error::ActivityRateError(value.into()))
    }
}

impl From<customer::DomainError> for crate::Error {
    fn from(value: customer::DomainError) -> Self {
        Self::TenantDatabaseError(Error::CustomerError(value.into()))
    }
}

impl From<project::DomainError> for crate::Error {
    fn from(value: project::DomainError) -> Self {
        Self::TenantDatabaseError(Error::ProjectError(value.into()))
    }
}

impl From<project_rate::DomainError> for crate::Error {
    fn from(value: project_rate::DomainError) -> Self {
        Self::TenantDatabaseError(Error::ProjectRateError(value.into()))
    }
}

impl From<tag::DomainError> for crate::Error {
    fn from(value: tag::DomainError) -> Self {
        Self::TenantDatabaseError(Error::TagError(value.into()))
    }
}

impl From<timesheet::DomainError> for crate::Error {
    fn from(value: timesheet::DomainError) -> Self {
        Self::TenantDatabaseError(Error::TimesheetError(value.into()))
    }
}
