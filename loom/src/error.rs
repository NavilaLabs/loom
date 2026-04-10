//! Shared error types returned by loom controllers.
//!
//! Presentation layers can downcast these to generate appropriate HTTP
//! responses (e.g. 422 Unprocessable Entity for `ValidationError`).

use thiserror::Error;

/// A domain-level input validation failure.
///
/// Controllers return this (via `anyhow::Error`) when the caller supplies
/// data that violates domain invariants (empty name, wrong format, etc.).
/// The presentation layer can downcast to this type and map it to a 422
/// response, while leaving all other errors as 500.
#[derive(Debug, Error)]
#[error("{0}")]
pub struct ValidationError(pub String);

impl ValidationError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}
