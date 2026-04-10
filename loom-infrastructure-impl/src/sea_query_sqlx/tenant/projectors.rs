use async_trait::async_trait;
use eventually_projection::{Projector, RawEvent};

use crate::{
    ConnectedTenantPool,
    sea_query_sqlx::tenant::{
        activity::projectors::ActivityProjector, activity_rate::projectors::ActivityRateProjector,
        customer::projectors::CustomerProjector, project::projectors::ProjectProjector,
        project_rate::projectors::ProjectRateProjector, tag::projectors::TagProjector,
        timesheet::projectors::TimesheetProjector,
    },
};

/// A single projector that dispatches each event to all tenant sub-projectors
/// in a fixed, deterministic order.
///
/// Running all projectors under one [`ProjectionRunner`] with one shared
/// checkpoint guarantees that events are applied sequentially across every
/// projection table, preventing FK race conditions (e.g. a `ProjectCreated`
/// event being applied before the corresponding `CustomerCreated` has been
/// committed).
pub struct TenantProjector {
    customer: CustomerProjector,
    project: ProjectProjector,
    activity: ActivityProjector,
    timesheet: TimesheetProjector,
    tag: TagProjector,
    project_rate: ProjectRateProjector,
    activity_rate: ActivityRateProjector,
}

impl TenantProjector {
    #[must_use]
    pub fn new(pool: ConnectedTenantPool) -> Self {
        Self {
            customer: CustomerProjector::new(pool.clone()),
            project: ProjectProjector::new(pool.clone()),
            activity: ActivityProjector::new(pool.clone()),
            timesheet: TimesheetProjector::new(pool.clone()),
            tag: TagProjector::new(pool.clone()),
            project_rate: ProjectRateProjector::new(pool.clone()),
            activity_rate: ActivityRateProjector::new(pool),
        }
    }
}

#[async_trait]
impl Projector for TenantProjector {
    type Error = crate::Error;

    async fn handle(&mut self, event: RawEvent) -> Result<(), Self::Error> {
        self.customer.handle(event.clone()).await?;
        self.project.handle(event.clone()).await?;
        self.activity.handle(event.clone()).await?;
        self.timesheet.handle(event.clone()).await?;
        self.tag.handle(event.clone()).await?;
        self.project_rate.handle(event.clone()).await?;
        self.activity_rate.handle(event).await?;
        Ok(())
    }
}
