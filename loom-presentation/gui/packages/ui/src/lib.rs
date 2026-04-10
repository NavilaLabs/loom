//! This crate contains all shared UI for the workspace.

use dioxus::prelude::*;

pub mod components;
pub mod form_machine;
pub mod layouts;
pub mod views;

pub const FAVICON: Asset = asset!("/assets/favicon.svg");

/// Global shared state for the currently running timesheet.
/// Provided by the top-level `Layout` and consumed by Sidebar, Dashboard, and Timesheets.
pub type RunningTimer = Signal<Option<api::timesheet::TimesheetDto>>;

/// Global shared elapsed-seconds counter for the running timer.
/// Updated by a single coroutine in `Layout`; all components read from this.
pub type RunningElapsed = Signal<u64>;
