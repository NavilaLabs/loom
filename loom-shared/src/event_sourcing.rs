use serde::{Serialize, de::DeserializeOwned};

pub trait Aggregate {
    const TYPE: &'static str;
    const SCHEMA_VERSION: u32;

    type Event: Serialize + DeserializeOwned;

    fn apply(&mut self, event: Self::Event);
}

pub trait Projection {
    const TYPE: &'static str;
    const SCHEMA_VERSION: u32;

    type Event: Serialize + DeserializeOwned;

    fn apply(&mut self, event: Self::Event);
}
