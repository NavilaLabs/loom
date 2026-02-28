pub trait UserRepository
where
    Self: Sized,
{
    fn create_user(name: &str, secret: &str) -> Result<Self, crate::admin::user::Error>;
}
