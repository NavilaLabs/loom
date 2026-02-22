use async_trait::async_trait;
use modules::{EventEnvelope, EventType, EventVersion};
use serde::Serialize;

use crate::{
    EventRecord, ImplError,
    event_store::{GetLastAggregateRecord, Insert},
};

#[async_trait]
pub trait Append: ImplError {
    async fn append<T>(&self, envelope: EventEnvelope<T>) -> Result<(), <Self as ImplError>::Error>
    where
        T: EventType + EventVersion + Serialize + Send + Sync;
}

#[async_trait]
impl<T> Append for T
where
    T: GetLastAggregateRecord + Insert + ImplError + Send + Sync,
{
    async fn append<U>(&self, envelope: EventEnvelope<U>) -> Result<(), <Self as ImplError>::Error>
    where
        U: EventType + EventVersion + Serialize + Send + Sync,
    {
        let aggregate_id = envelope.get_aggregate().get_id();

        let last_record = self
            .get_last_aggregate_record(envelope.get_aggregate().get_type(), aggregate_id)
            .await?;

        let previous_hash: Vec<u8> = match last_record {
            Some(record) => record.get_hash().to_owned(),
            None => vec![],
        };

        let new_record = EventRecord::from_envelope(envelope, previous_hash)?;

        self.insert(new_record).await?;

        Ok(())
    }
}
