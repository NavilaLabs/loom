use eventually::message::Message;
use serde::{Deserialize, Serialize};

use crate::tenant::customer::CustomerId;
use crate::tenant::project::ProjectId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProjectEvent {
    Created {
        id: ProjectId,
        customer_id: CustomerId,
        name: String,
    },
    Updated {
        name: String,
        comment: Option<String>,
        order_number: Option<String>,
        visible: bool,
        billable: bool,
    },
    BudgetUpdated {
        time_budget: Option<i32>,
        money_budget: Option<i64>,
        budget_is_monthly: bool,
    },
}

impl Message for ProjectEvent {
    fn name(&self) -> &'static str {
        match self {
            ProjectEvent::Created { .. } => "ProjectCreated",
            ProjectEvent::Updated { .. } => "ProjectUpdated",
            ProjectEvent::BudgetUpdated { .. } => "ProjectBudgetUpdated",
        }
    }
}
