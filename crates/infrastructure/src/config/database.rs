use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    uri: Uri,
    users: Users,
    databases: Databases,
}

impl Database {
    pub fn get_postgres_admin_uri(&self) -> super::Result<Url> {
        Ok(format!(
            "postgres://{}:{}@{}:{}/",
            self.users.get_admin().get_credentials().get_username(),
            self.users.get_admin().get_credentials().get_password(),
            self.uri.get_host(),
            self.uri.get_port(),
        )
        .parse()?)
    }

    pub fn get_postgres_tenant_uri(&self, tenant_token: &str) -> super::Result<Url> {
        Ok(format!(
            "postgres://{}@{}:{}/",
            format!(
                "{}{}",
                self.users
                    .get_tenant()
                    .get_credentials()
                    .get_username_prefix(),
                tenant_token
            ),
            self.uri.get_host(),
            self.uri.get_port(),
        )
        .parse()?)
    }

    pub fn get_uri(&self) -> &Uri {
        &self.uri
    }

    pub fn get_users(&self) -> &Users {
        &self.users
    }

    pub fn get_databases(&self) -> &Databases {
        &self.databases
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uri {
    host: String,
    port: u16,
}

impl Uri {
    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Users {
    admin: AdminUser,
    tenant: TenantUser,
}

impl Users {
    pub fn get_admin(&self) -> &AdminUser {
        &self.admin
    }

    pub fn get_tenant(&self) -> &TenantUser {
        &self.tenant
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Databases {
    admin: AdminDatabase,
    tenant: TenantDatabase,
}

impl Databases {
    pub fn get_admin(&self) -> &AdminDatabase {
        &self.admin
    }

    pub fn get_tenant(&self) -> &TenantDatabase {
        &self.tenant
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminDatabase {
    name: String,
}

impl AdminDatabase {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantDatabase {
    name_prefix: String,
}

impl TenantDatabase {
    pub fn get_name_prefix(&self) -> &str {
        &self.name_prefix
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "admin")]
pub struct AdminUser {
    credentials: AdminCredentials,
}

impl AdminUser {
    pub fn get_credentials(&self) -> &AdminCredentials {
        &self.credentials
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tenant")]
pub struct TenantUser {
    credentials: TenantCredentials,
}

impl TenantUser {
    pub fn get_credentials(&self) -> &TenantCredentials {
        &self.credentials
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminCredentials {
    username: String,
    password: String,
}

impl AdminCredentials {
    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantCredentials {
    username_prefix: String,
}

impl TenantCredentials {
    pub fn get_username_prefix(&self) -> &str {
        &self.username_prefix
    }
}

pub struct Pool {
    max_size: u32,
    min_size: u32,
    timeout_seconds: u64,
}

impl Pool {
    pub fn get_max_size(&self) -> u32 {
        self.max_size
    }

    pub fn get_min_size(&self) -> u32 {
        self.min_size
    }

    pub fn get_timeout_seconds(&self) -> u64 {
        self.timeout_seconds
    }
}
