pub mod aggregates;
pub mod events;
pub mod interfaces;
pub mod value_objects;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    AggregateError(#[from] aggregates::Error),
}
