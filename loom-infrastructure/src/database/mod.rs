mod initialize;
pub use initialize::*;
pub mod migrate;
pub use migrate::*;
mod tenant_database_name_builder;
pub use tenant_database_name_builder::{
    Builder as TenantDatabaseNameBuilder, ConcreteBuilder as TenantDatabaseNameConcreteBuilder,
    Director as TenantDatabaseNameDirector,
};
pub mod database_uri_factory;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No tenant token provided")]
    NoTenantTokenProvided,
}
