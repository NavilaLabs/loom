use async_trait::async_trait;

use crate::tenant::tag::domain::aggregates::Tag;
use eventually::aggregate::repository::{Getter, Saver};

#[async_trait]
pub trait TagRepository: Getter<Tag> + Saver<Tag> + Send + Sync {}
