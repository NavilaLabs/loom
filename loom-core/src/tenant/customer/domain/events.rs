use eventually::message::Message;
use serde::{Deserialize, Serialize};

use crate::tenant::customer::CustomerId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CustomerEvent {
    Created {
        id: CustomerId,
        name: String,
        currency: String,
        timezone: String,
    },
    Updated {
        name: String,
        comment: Option<String>,
        currency: String,
        timezone: String,
        country: Option<String>,
        visible: bool,
    },
    BudgetUpdated {
        time_budget: Option<i32>,
        money_budget: Option<i64>,
        budget_is_monthly: bool,
    },
}

impl Message for CustomerEvent {
    fn name(&self) -> &'static str {
        match self {
            CustomerEvent::Created { .. } => "CustomerCreated",
            CustomerEvent::Updated { .. } => "CustomerUpdated",
            CustomerEvent::BudgetUpdated { .. } => "CustomerBudgetUpdated",
        }
    }
}
