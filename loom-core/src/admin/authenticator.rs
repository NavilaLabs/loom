use std::fmt::Debug;

pub trait AuthenticationStrategy {
    type Error: Debug;

    fn authenticate<U>(&self, secret: &str, name: Option<&str>) -> Result<Option<U>, Self::Error>;
}

pub struct Authenticator<T: AuthenticationStrategy> {
    strategy: T,
}

impl<T: AuthenticationStrategy> Authenticator<T> {
    pub fn new(strategy: T) -> Self {
        Self { strategy }
    }

    pub fn authenticate<U>(&self, secret: &str, name: Option<&str>) -> Result<Option<U>, T::Error> {
        self.strategy.authenticate(secret, name)
    }
}
