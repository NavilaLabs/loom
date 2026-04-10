use validator::Validate;

#[derive(Clone, Validate)]
pub struct CreateProjectInput {
    #[validate(length(min = 1, max = 255, message = "Name must not be empty"))]
    pub name: String,
}

#[derive(Clone, Validate)]
pub struct UpdateProjectInput {
    #[validate(length(min = 1, max = 255, message = "Name must not be empty"))]
    pub name: String,
}
