#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    UserCreated(UserCreated),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserCreated {
    name: String,
    secret: String,
}

impl UserCreated {
    pub fn new(name: String, secret: String) -> Self {
        UserCreated { name, secret }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_secret(&self) -> &str {
        &self.secret
    }
}

#[cfg(feature = "eventually")]
mod eventually {
    use eventually::message::Message;

    use crate::admin::user::events::Event;

    impl Message for Event {
        fn name(&self) -> &'static str {
            match self {
                Event::UserCreated(_) => "UserCreated",
            }
        }
    }
}
