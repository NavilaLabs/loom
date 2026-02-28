#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateUser {
    pub name: String,
    pub password: String,
}

#[cfg(feature = "eventually")]
mod eventually {
    use async_trait::async_trait;
    use eventually::{aggregate::Repository, command, message::Message};

    use crate::admin::user::{
        self, UserService, commands::CreateUser, interfaces::repositories::UserRepository,
    };

    impl Message for CreateUser {
        fn name(&self) -> &'static str {
            "CreateUser"
        }
    }

    #[async_trait]
    impl<R> command::Handler<CreateUser> for UserService<R>
    where
        R: Repository<user::domain::aggregates::User>,
    {
        type Error = user::Error;

        async fn handle(&self, command: command::Envelope<CreateUser>) -> Result<(), Self::Error> {
            let command = command.message;
            let mut user = user::infrastructure::repositories::UserRoot::create_user(
                &command.name,
                &command.password,
            )?;
            self.get_repository().save(&mut user).await?;
            Ok(())
        }
    }
}
