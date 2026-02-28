pub mod commands;
pub mod queries;

#[cfg(feature = "eventually")]
pub(crate) use eventually::*;

#[cfg(feature = "eventually")]
mod eventually {
    use eventually::aggregate::Repository;

    use crate::admin::user;

    pub struct UserService<R>
    where
        R: Repository<user::domain::aggregates::User>,
    {
        repository: R,
    }

    impl<R> UserService<R>
    where
        R: Repository<user::domain::aggregates::User>,
    {
        pub fn get_repository(&self) -> &R {
            &self.repository
        }
    }

    impl<R> From<R> for UserService<R>
    where
        R: Repository<user::domain::aggregates::User>,
    {
        fn from(repository: R) -> Self {
            Self { repository }
        }
    }
}
