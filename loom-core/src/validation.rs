//! Shared validation utilities used by both loom controllers and the GUI frontend.

pub use validator::{Validate, ValidationErrors};

/// Format `ValidationErrors` into a single user-friendly string.
///
/// Collects only the human-readable `message` from each field error and
/// joins them with "; ". Falls back to the full debug representation if
/// a field error carries no message.
#[must_use] 
pub fn validation_summary(e: &ValidationErrors) -> String {
    let messages: Vec<String> = e
        .field_errors()
        .into_values()
        .flat_map(|errs| errs.iter())
        .filter_map(|fe| fe.message.as_deref().map(str::to_owned))
        .collect();

    if messages.is_empty() {
        e.to_string()
    } else {
        messages.join("; ")
    }
}
