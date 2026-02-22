use async_trait::async_trait;
use modules::shared::value_objects::{AggregateId, AggregateType};

use crate::{EventRecord, ImplError};

#[async_trait]
pub trait GetLastAggregateRecord: ImplError {
    async fn get_last_aggregate_record(
        &self,
        aggregate_type: &AggregateType,
        aggregate_id: &AggregateId,
    ) -> Result<Option<EventRecord>, <Self as ImplError>::Error>;
}
