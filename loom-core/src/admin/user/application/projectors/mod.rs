use async_trait::async_trait;
use eventually_projection::{Projector, RawEvent};
use loom_infrastructure_impl::{Pool, ScopeAdmin, StateConnected};
use serde_json::Value as JsonValue;
use uuid::Uuid;

pub struct UserProjector {
    pool: Pool<ScopeAdmin, StateConnected>,
}

impl UserProjector {
    pub fn new(pool: Pool<ScopeAdmin, StateConnected>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Projector for UserProjector {
    type Error = crate::Error;

    async fn handle(&mut self, event: RawEvent) -> Result<(), Self::Error> {
        let parsed: JsonValue = serde_json::from_slice(&event.payload_bytes)?;

        match event.event_type.as_str() {
            "UserCreated" => {
                let id: Uuid = parsed["id"].as_str().unwrap().parse()?;
                let name: String = parsed["name"].as_str().unwrap().to_string();

                todo!()
            }
            _ => Ok(()),
        }
    }
}
