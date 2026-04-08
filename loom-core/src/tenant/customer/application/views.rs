use crate::tenant::customer::CustomerId;

#[derive(Debug, Clone)]
pub struct CustomerView {
    id: CustomerId,
    name: String,
    comment: Option<String>,
    currency: String,
    timezone: String,
    country: Option<String>,
    visible: bool,
    time_budget: Option<i32>,
    money_budget: Option<i64>,
    budget_is_monthly: bool,
}

impl CustomerView {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: CustomerId,
        name: String,
        comment: Option<String>,
        currency: String,
        timezone: String,
        country: Option<String>,
        visible: bool,
        time_budget: Option<i32>,
        money_budget: Option<i64>,
        budget_is_monthly: bool,
    ) -> Self {
        Self { id, name, comment, currency, timezone, country, visible, time_budget, money_budget, budget_is_monthly }
    }

    pub fn get_id(&self) -> &CustomerId { &self.id }
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_comment(&self) -> Option<&str> { self.comment.as_deref() }
    pub fn get_currency(&self) -> &str { &self.currency }
    pub fn get_timezone(&self) -> &str { &self.timezone }
    pub fn get_country(&self) -> Option<&str> { self.country.as_deref() }
    pub fn is_visible(&self) -> bool { self.visible }
    pub fn get_time_budget(&self) -> Option<i32> { self.time_budget }
    pub fn get_money_budget(&self) -> Option<i64> { self.money_budget }
    pub fn is_budget_monthly(&self) -> bool { self.budget_is_monthly }
}
