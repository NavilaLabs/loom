use crate::shared::Id;

#[derive(Debug, Clone)]
pub struct User {
    id: Id,
    name: String,
    secret: String,
}

#[cfg(feature = "eventually")]
mod eventually {
    use eventually::aggregate::Aggregate;
    use uuid::{ContextV7, Timestamp};

    use crate::{
        admin::user::{self, aggregates::User},
        shared::Id,
    };

    impl Aggregate for User {
        type Id = Id;
        type Event = user::events::Event;
        type Error = user::Error;

        fn type_name() -> &'static str {
            "User"
        }

        fn aggregate_id(&self) -> &Self::Id {
            &self.id
        }

        fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
            match state {
                None => match event {
                    user::events::Event::UserCreated(user_created) => Ok(User {
                        id: Id::new(Timestamp::now(ContextV7::new())),
                        name: user_created.get_name().to_string(),
                        secret: user_created.get_secret().to_string(),
                    }),
                },
                Some(user) => todo!(),
            }
        }
    }
}
