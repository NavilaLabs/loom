use url::Url;

use crate::{
    config::CONFIG,
    database::{
        TenantDatabaseNameBuilder, TenantDatabaseNameConcreteBuilder, TenantDatabaseNameDirector,
    },
};

pub trait DatabaseUri {
    fn get_uri(&self, database_type: &str, tenant_token: Option<&str>)
    -> Result<Url, crate::Error>;

    /// Ensures that the database URI has a `.sqlite` extension for SQLite databases.
    fn ensure_sqlite_extension(
        &self,
        database_type: &str,
        database_uri: Url,
    ) -> Result<Url, crate::Error> {
        if database_type == "sqlite" {
            let mut uri = database_uri.to_string();
            if !uri.ends_with(".sqlite") {
                uri.push_str(".sqlite");
            }
            return Ok(Url::parse(&uri)?);
        }
        Ok(database_uri)
    }
}

pub enum DatabaseScope {
    Admin,
    Tenant,
}

pub struct AdminDatabaseUri;

impl DatabaseUri for AdminDatabaseUri {
    fn get_uri(
        &self,
        database_type: &str,
        _tenant_token: Option<&str>,
    ) -> Result<Url, crate::Error> {
        let base_uri = CONFIG.get_database().get_base_uri();
        let admin_database_name = CONFIG.get_database().get_databases().get_admin().get_name();
        let admin_uri = format!("{}/{}", base_uri, admin_database_name);
        let admin_uri = self.ensure_sqlite_extension(database_type, Url::parse(&admin_uri)?)?;

        Ok(admin_uri)
    }
}

pub struct TenantDatabaseUri;

impl DatabaseUri for TenantDatabaseUri {
    fn get_uri(
        &self,
        database_type: &str,
        tenant_token: Option<&str>,
    ) -> Result<Url, crate::Error> {
        let base_uri = CONFIG.get_database().get_base_uri();
        let tenant_token = tenant_token.map_or_else(
            || Err(crate::database::Error::NoTenantTokenProvided),
            |token| Ok(token),
        )?;
        let mut database_name_builder = TenantDatabaseNameConcreteBuilder::new();
        TenantDatabaseNameDirector::construct(&mut database_name_builder, tenant_token);
        let database_name = database_name_builder.get_tenant_database_name();
        let tenant_uri = format!("{}/{}", base_uri, database_name);
        let tenant_uri = self.ensure_sqlite_extension(database_type, Url::parse(&tenant_uri)?)?;

        Ok(tenant_uri)
    }
}

pub struct Factory;

impl Factory {
    pub fn new_database_uri(database_scope: &DatabaseScope) -> Box<dyn DatabaseUri> {
        match database_scope {
            DatabaseScope::Admin => Box::new(AdminDatabaseUri),
            DatabaseScope::Tenant => Box::new(TenantDatabaseUri),
        }
    }
}
