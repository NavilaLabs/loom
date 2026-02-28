#[cfg(feature = "eventually")]
pub use crate::admin::user::infrastructure::repositories::eventually::*;

#[cfg(feature = "eventually")]
mod eventually {
    use std::ops::{Deref, DerefMut};

    use eventually::aggregate::Root;

    use crate::admin::user::{
        self, aggregates::User, events::UserCreated, interfaces::repositories::UserRepository,
    };

    pub struct UserRoot(Root<User>);

    impl From<Root<User>> for UserRoot {
        fn from(root: Root<User>) -> Self {
            UserRoot(root)
        }
    }

    impl From<UserRoot> for Root<User> {
        fn from(root: UserRoot) -> Self {
            root.0
        }
    }

    impl Deref for UserRoot {
        type Target = Root<User>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for UserRoot {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl UserRepository for UserRoot {
        fn create_user(name: &str, secret: &str) -> Result<Self, crate::admin::user::Error> {
            Root::<User>::record_new(
                user::events::Event::UserCreated(UserCreated::new(
                    name.to_string(),
                    secret.to_string(),
                ))
                .into(),
            )
            .map(Self)
        }
    }
}
