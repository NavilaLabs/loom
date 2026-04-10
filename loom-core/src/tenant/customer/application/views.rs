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
    #[must_use]
    pub const fn new(
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
        Self {
            id,
            name,
            comment,
            currency,
            timezone,
            country,
            visible,
            time_budget,
            money_budget,
            budget_is_monthly,
        }
    }

    #[must_use]
    pub const fn get_id(&self) -> &CustomerId {
        &self.id
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
    pub fn get_currency(&self) -> &str {
        &self.currency
    }
    #[must_use]
    pub fn get_timezone(&self) -> &str {
        &self.timezone
    }
    #[must_use]
    pub fn get_country(&self) -> Option<&str> {
        self.country.as_deref()
    }
    #[must_use]
    pub const fn is_visible(&self) -> bool {
        self.visible
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
