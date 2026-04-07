use anyhow::Result;
use loom_core::admin::{
    authenticator::Authenticator,
    user::LoginQuery,
};
use loom_infrastructure_impl::{
    Pool,
    admin::{authentication::jwt::JwtAuthentication, user::repositories::UserRepository},
};

pub async fn login_user(email: String, password: String) -> Result<String> {
    let pool = Pool::connect_admin().await?;
    let user_repo = UserRepository::from_pool(pool).await?;
    let query = LoginQuery::new(user_repo, Authenticator::new(JwtAuthentication));
    let token = query
        .login(&email, &password)
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok(token)
}
