//! Permission name constants following the `<aggregate>.<action>` convention.
//!
//! These are the canonical string keys stored in the `permissions` table.
//! The "admin" workspace role bypasses all permission checks — only regular
//! users with an explicit role or direct grant need these constants.
//!
//! Naming rule: `<aggregate>.<action>`, e.g. `customer.create`.
//! Event names are internal implementation details and are intentionally
//! kept separate from permission names.

// Customer domain
pub const CUSTOMER_CREATE: &str = "customer.create";
pub const CUSTOMER_UPDATE: &str = "customer.update";

// Project domain
pub const PROJECT_CREATE: &str = "project.create";
pub const PROJECT_UPDATE: &str = "project.update";

// Activity domain
pub const ACTIVITY_CREATE: &str = "activity.create";
pub const ACTIVITY_UPDATE: &str = "activity.update";

// Timesheet domain
pub const TIMESHEET_CREATE: &str = "timesheet.create";
pub const TIMESHEET_UPDATE: &str = "timesheet.update";
pub const TIMESHEET_EXPORT: &str = "timesheet.export";

// Cross-cutting
pub const TAG_MANAGE: &str = "tag.manage";
pub const RATE_MANAGE: &str = "rate.manage";

/// Every permission that must be seeded in the database.
/// Used by migrations and initial setup logic.
pub const ALL: &[&str] = &[
    CUSTOMER_CREATE,
    CUSTOMER_UPDATE,
    PROJECT_CREATE,
    PROJECT_UPDATE,
    ACTIVITY_CREATE,
    ACTIVITY_UPDATE,
    TIMESHEET_CREATE,
    TIMESHEET_UPDATE,
    TIMESHEET_EXPORT,
    TAG_MANAGE,
    RATE_MANAGE,
];
