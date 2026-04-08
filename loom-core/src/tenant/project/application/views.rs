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
    pub fn new(
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
        Self { id, customer_id, name, comment, order_number, visible, billable, time_budget, money_budget, budget_is_monthly }
    }

    pub fn get_id(&self) -> &ProjectId { &self.id }
    pub fn get_customer_id(&self) -> &CustomerId { &self.customer_id }
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_comment(&self) -> Option<&str> { self.comment.as_deref() }
    pub fn get_order_number(&self) -> Option<&str> { self.order_number.as_deref() }
    pub fn is_visible(&self) -> bool { self.visible }
    pub fn is_billable(&self) -> bool { self.billable }
    pub fn get_time_budget(&self) -> Option<i32> { self.time_budget }
    pub fn get_money_budget(&self) -> Option<i64> { self.money_budget }
    pub fn is_budget_monthly(&self) -> bool { self.budget_is_monthly }
}
