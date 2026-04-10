//! This crate contains all shared UI for the workspace.

use dioxus::prelude::*;

pub mod components;
pub mod form_machine;
pub mod formatting;
pub mod hooks;
pub mod layouts;
pub mod views;

pub const FAVICON: Asset = asset!("/assets/favicon.svg");

/// Global shared state for the currently running timesheet.
/// Provided by the top-level `Layout` and consumed by Sidebar, Dashboard, and Timesheets.
pub type RunningTimer = Signal<Option<api::timesheet::TimesheetDto>>;

/// Global shared elapsed-seconds counter for the running timer.
/// Updated by a single coroutine in `Layout`; all components read from this.
pub type RunningElapsed = Signal<u64>;

/// User-level display settings (timezone, date format, language).
/// Loaded once in `Layout` and available to every component via context.
pub type UserSettings = Signal<api::settings::UserSettingsDto>;

/// Workspace-level settings (name, timezone, date format, currency, week start).
/// Loaded once in `Layout` and available to every component via context.
pub type WorkspaceSettings = Signal<api::settings::WorkspaceSettingsDto>;

/// Global cache of all customers. Pre-populated in `Layout` so views start with data.
pub type CustomersCache = Signal<Vec<api::customer::CustomerDto>>;

/// Global cache of all projects. Pre-populated in `Layout` so views start with data.
pub type ProjectsCache = Signal<Vec<api::project::ProjectDto>>;

/// Global cache of all activities. Pre-populated in `Layout` so views start with data.
pub type ActivitiesCache = Signal<Vec<api::activity::ActivityDto>>;

/// Global cache of all tags. Pre-populated in `Layout` so views start with data.
pub type TagsCache = Signal<Vec<api::tag::TagDto>>;

/// Global cache of recent timesheets. Pre-populated in `Layout` so views start with data.
pub type TimesheetsCache = Signal<Vec<api::timesheet::TimesheetDto>>;
