use async_trait::async_trait;

use crate::{EventRecord, ImplError};

#[async_trait]
pub trait Insert: ImplError {
    async fn insert(&self, record: EventRecord) -> Result<(), <Self as ImplError>::Error>;
}
