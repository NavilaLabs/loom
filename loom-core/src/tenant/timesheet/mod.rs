pub(crate) mod application;
pub(crate) mod domain;

pub use application::{commands::TimesheetCommand, views::TimesheetView};
pub use domain::{
    Error as DomainError,
    aggregates::{Timesheet, TimesheetId},
    events::TimesheetEvent,
    interfaces::TimesheetRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    DomainError(#[from] domain::Error),
}
