use std::ops::DerefMut;

use async_trait::async_trait;
use domain::tenant::value_objects::TenantToken;

use crate::{
    config::CONFIG,
    database::{
        Error, Initialize, Pool, ScopeDefault, StateConnected, sea_query_sqlx::AcquiredConnection,
    },
};

#[async_trait]
impl Initialize<AcquiredConnection> for Pool<ScopeDefault, StateConnected> {
    type Error = Error;

    async fn is_initialized(&self, _connection: &AcquiredConnection) -> Result<bool, Error> {
        todo!()
    }

    async fn initialize_admin_database(
        &self,
        connection: &mut AcquiredConnection,
    ) -> Result<(), <Self as Initialize<AcquiredConnection>>::Error> {
        let database_name = CONFIG.get_database().get_databases().get_admin().get_name();

        let query = format!(r#"CREATE DATABASE "{}""#, database_name);

        sqlx::query(query.as_str())
            .execute(connection.deref_mut())
            .await?;

        Ok(())
    }

    async fn initialize_tenant_database(
        &self,
        connection: &mut AcquiredConnection,
        tenant_token: Option<&TenantToken>,
    ) -> Result<(), <Self as Initialize<AcquiredConnection>>::Error> {
        let tenant_database_prefix = CONFIG
            .get_database()
            .get_databases()
            .get_tenant()
            .get_name_prefix();
        let tenant_token = match tenant_token {
            Some(token) => token.as_ref(),
            None => "template".into(),
        };

        let query = format!(
            r#"CREATE DATABASE "{}_{}""#,
            tenant_database_prefix, tenant_token
        );
        sqlx::query(query.as_str())
            .execute(connection.deref_mut())
            .await?;

        Ok(())
    }
}
