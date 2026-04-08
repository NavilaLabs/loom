use async_trait::async_trait;

use crate::tenant::timesheet::domain::aggregates::Timesheet;
use eventually::aggregate::repository::{Getter, Saver};

#[async_trait]
pub trait TimesheetRepository: Getter<Timesheet> + Saver<Timesheet> + Send + Sync {}
