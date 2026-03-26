use crate::admin::user::{UserId, UserRepository, UserRoot};

pub async fn create_user(
    repository: &impl UserRepository,
    id: UserId,
    email: String,
) -> Result<(), crate::Error> {
    let mut root = UserRoot::create(id, email)?;
    repository.save(&mut root).await?;

    Ok(())
}

#[cfg(feature = "sqlite")]
#[cfg(test)]
mod test {
    use eventually::aggregate::repository::Getter;
    use eventually_any::snapshot::Repository;
    use loom_tests::{
        get_admin_pool, get_default_pool, refresh_databases, test_database_lifecycle,
    };
    use with_lifecycle::with_lifecycle;

    use crate::admin::user::{User, UserEvent, create_user};

    #[with_lifecycle(test_database_lifecycle)]
    #[tokio::test]
    async fn test_create_user() {
        let default_pool = get_default_pool().await.unwrap();
        refresh_databases(&default_pool, "test_token")
            .await
            .unwrap();

        let admin_pool = get_admin_pool().await.unwrap();

        let user_repository: Repository<User, _, _> = Repository::new(
            admin_pool.into_pool(),
            eventually::serde::Json::<User>::default(),
            eventually::serde::Json::<UserEvent>::default(),
        )
        .await
        .unwrap();

        create_user(
            &user_repository,
            "019d0ce8-facb-7c90-b9d7-287ae4f17c91".parse().unwrap(),
            "loom".to_string(),
        )
        .await
        .unwrap();

        let user = user_repository
            .get(&"019d0ce8-facb-7c90-b9d7-287ae4f17c91".parse().unwrap())
            .await
            .unwrap();

        assert_eq!(user.name(), "loom");
    }
}
