use std::sync::LazyLock;

use eventually::version::{ConflictError, Version};
use regex::Regex;

pub mod aggragte;
pub mod event;

static CONFLICT_ERROR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"version check failed, expected: (?P<expected>\d), got: (?P<got>\d)")
        .expect("regex compiles successfully")
});

pub(crate) fn check_for_conflict_error(err: &sqlx::Error) -> Option<ConflictError> {
    fn capture_to_version(captures: &regex::Captures, name: &'static str) -> Version {
        let v: i32 = captures
            .name(name)
            .expect("field is captured")
            .as_str()
            .parse::<i32>()
            .expect("field should be a valid integer");

        #[allow(clippy::cast_sign_loss)]
        {
            v as Version
        }
    }

    if let sqlx::Error::Database(db_err) = err {
        return CONFLICT_ERROR_REGEX
            .captures(db_err.message())
            .map(|captures| ConflictError {
                actual: capture_to_version(&captures, "got"),
                expected: capture_to_version(&captures, "expected"),
            });
    }

    None
}
