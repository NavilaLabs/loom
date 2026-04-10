use validator::Validate;

#[derive(Clone, Validate)]
pub struct CreateTagInput {
    #[validate(length(min = 1, max = 100, message = "Tag name must not be empty"))]
    pub name: String,
}

#[derive(Clone, Validate)]
pub struct RenameTagInput {
    #[validate(length(min = 1, max = 100, message = "Tag name must not be empty"))]
    pub name: String,
}
