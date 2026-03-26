pub mod commands;
pub mod queries;
pub mod use_cases;
pub mod views;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {}
