use crate::tenant::customer::CustomerId;
use crate::tenant::project::ProjectId;

#[derive(Debug, Clone)]
pub struct ProjectView {
    id: ProjectId,
    customer_id: CustomerId,
    name: String,
    comment: Option<String>,
    order_number: Option<String>,
    visible: bool,
    billable: bool,
    time_budget: Option<i32>,
    money_budget: Option<i64>,
    budget_is_monthly: bool,
}

impl ProjectView {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        id: ProjectId,
        customer_id: CustomerId,
        name: String,
        comment: Option<String>,
        order_number: Option<String>,
        visible: bool,
        billable: bool,
        time_budget: Option<i32>,
        money_budget: Option<i64>,
        budget_is_monthly: bool,
    ) -> Self {
        Self {
            id,
            customer_id,
            name,
            comment,
            order_number,
            visible,
            billable,
            time_budget,
            money_budget,
            budget_is_monthly,
        }
    }

    #[must_use]
    pub const fn get_id(&self) -> &ProjectId {
        &self.id
    }
    #[must_use]
    pub const fn get_customer_id(&self) -> &CustomerId {
        &self.customer_id
    }
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }
    #[must_use]
    pub fn get_comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }
    #[must_use]
    pub fn get_order_number(&self) -> Option<&str> {
        self.order_number.as_deref()
    }
    #[must_use]
    pub const fn is_visible(&self) -> bool {
        self.visible
    }
    #[must_use]
    pub const fn is_billable(&self) -> bool {
        self.billable
    }
    #[must_use]
    pub const fn get_time_budget(&self) -> Option<i32> {
        self.time_budget
    }
    #[must_use]
    pub const fn get_money_budget(&self) -> Option<i64> {
        self.money_budget
    }
    #[must_use]
    pub const fn is_budget_monthly(&self) -> bool {
        self.budget_is_monthly
    }
}
