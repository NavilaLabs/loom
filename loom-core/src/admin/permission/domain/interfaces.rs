use crate::admin::permission::domain::aggregates::Permission;
use eventually::aggregate::repository::{Getter, Saver};

pub trait PermissionRepository: Getter<Permission> + Saver<Permission> + Send + Sync {}
