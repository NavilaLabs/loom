use std::sync::Arc;

use anyhow::Result;
use loom_core::admin::user::{UserCommand, UserId, UserRepository};

pub struct UserController<R: UserRepository> {
    repository: Arc<R>,
    commands: Arc<UserCommand>,
    _queries: Arc<UserCommand>, // TODO: this is not a query root
}

impl<R: UserRepository> UserController<R> {
    pub const fn new(
        repository: Arc<R>,
        commands: Arc<UserCommand>,
        queries: Arc<UserCommand>,
    ) -> Self {
        Self {
            repository,
            commands,
            _queries: queries,
        }
    }

    pub async fn create_user(
        &self,
        id: UserId,
        name: String,
        email: String,
        password: String,
    ) -> Result<()> {
        let mut root = self.commands.create(id, name, email, password)?;
        self.repository.save(&mut root).await?;

        Ok(())
    }
}

// #[cfg(feature = "sqlite")]
// #[cfg(test)]
// mod test {
//     use eventually::aggregate::repository::Getter;
//     use eventually_any::snapshot::Repository;
//     use loom_tests::{
//         get_admin_pool, get_default_pool, refresh_databases, test_database_lifecycle,
//     };
//     use with_lifecycle::with_lifecycle;

//     use crate::admin::user::{User, UserEvent, create_user};

//     #[with_lifecycle(test_database_lifecycle)]
//     #[tokio::test]
//     async fn test_create_user() {
//         let default_pool = get_default_pool().await.unwrap();
//         refresh_databases(&default_pool, "test_token")
//             .await
//             .unwrap();

//         let admin_pool = get_admin_pool().await.unwrap();

//         let user_repository: Repository<User, _, _> = Repository::new(
//             admin_pool.into_pool(),
//             eventually::serde::Json::<User>::default(),
//             eventually::serde::Json::<UserEvent>::default(),
//         )
//         .await
//         .unwrap();

//         create_user(
//             &user_repository,
//             "019d0ce8-facb-7c90-b9d7-287ae4f17c91".parse().unwrap(),
//             "loom".to_string(),
//         )
//         .await
//         .unwrap();

//         let user = user_repository
//             .get(&"019d0ce8-facb-7c90-b9d7-287ae4f17c91".parse().unwrap())
//             .await
//             .unwrap();

//         assert_eq!(user.name(), "loom");
//     }
// }
