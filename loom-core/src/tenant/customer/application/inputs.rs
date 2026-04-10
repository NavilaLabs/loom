use validator::Validate;

#[derive(Clone, Validate)]
pub struct CreateCustomerInput {
    #[validate(length(min = 1, max = 255, message = "Name must not be empty"))]
    pub name: String,
    #[validate(length(equal = 3, message = "Currency must be a 3-letter ISO 4217 code"))]
    pub currency: String,
    #[validate(length(min = 1, message = "Timezone must not be empty"))]
    pub timezone: String,
}

#[derive(Clone, Validate)]
pub struct UpdateCustomerInput {
    #[validate(length(min = 1, max = 255, message = "Name must not be empty"))]
    pub name: String,
    #[validate(length(equal = 3, message = "Currency must be a 3-letter ISO 4217 code"))]
    pub currency: String,
    #[validate(length(min = 1, message = "Timezone must not be empty"))]
    pub timezone: String,
}
