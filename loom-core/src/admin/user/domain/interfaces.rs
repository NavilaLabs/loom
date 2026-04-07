use std::fmt::Debug;

use crate::admin::user::domain::aggregates::User;
use async_trait::async_trait;
use eventually::aggregate::repository::{Getter, Saver};

#[async_trait]
pub trait UserRepository: Getter<User> + Saver<User> + Send + Sync {
    type Error: Debug;

    async fn find_credentials_by_email(
        &self,
        email: &str,
    ) -> Result<Option<(String, String, String)>, Self::Error>;
}
