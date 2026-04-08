use anyhow::{Result, bail};
use loom_infrastructure_impl::Pool;

use crate::auth::CurrentUser;

/// Live permission checks against the projection tables.
///
/// All methods open a connection from the shared admin pool, run a single
/// SQL query, and return immediately — they never touch the event store.
pub struct AuthorizationService;

impl AuthorizationService {
    /// Returns `true` if the user holds an "admin" role in any workspace.
    ///
    /// Admins implicitly have every permission; call this before any
    /// fine-grained [`has_permission`] check to implement a short-circuit.
    pub async fn is_admin(user_id: &str) -> Result<bool> {
        let pool = Pool::connect_admin().await?.into_pool();
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)
             FROM projections__workspace_user_roles wur
             JOIN projections__workspace_roles wr
               ON wur.workspace_role_id = wr.id
             WHERE wur.user_id = $1
               AND wr.name = 'admin'",
        )
        .bind(user_id)
        .fetch_one(&pool)
        .await?;
        Ok(count > 0)
    }

    /// Returns `true` if the user has the named permission, either through
    /// a workspace role or as a directly-granted individual permission.
    pub async fn has_permission(user_id: &str, permission: &str) -> Result<bool> {
        let pool = Pool::connect_admin().await?.into_pool();

        // Check via role-level grant first.
        let via_role: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)
             FROM projections__workspace_user_roles wur
             JOIN projections__workspace_role_permissions wrp
               ON wur.workspace_role_id = wrp.workspace_role_id
             JOIN permissions p
               ON wrp.permission_id = p.id
             WHERE wur.user_id = $1
               AND p.name = $2",
        )
        .bind(user_id)
        .bind(permission)
        .fetch_one(&pool)
        .await?;

        if via_role > 0 {
            return Ok(true);
        }

        // Fall back to a direct per-user grant.
        let direct: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)
             FROM projections__workspace_user_permissions wup
             JOIN permissions p
               ON wup.permission_id = p.id
             WHERE wup.user_id = $1
               AND p.name = $2",
        )
        .bind(user_id)
        .bind(permission)
        .fetch_one(&pool)
        .await?;

        Ok(direct > 0)
    }

    /// Require that the requesting user is an admin, returning an error
    /// with a generic "forbidden" message if they are not.
    ///
    /// Use this in controllers that only admins may call.
    pub async fn require_admin(user: &CurrentUser) -> Result<()> {
        if Self::is_admin(&user.id).await? {
            Ok(())
        } else {
            bail!("forbidden")
        }
    }

    /// Require that the requesting user has the named permission (or is an
    /// admin), returning a generic "forbidden" error otherwise.
    pub async fn require_permission(user: &CurrentUser, permission: &str) -> Result<()> {
        if Self::is_admin(&user.id).await? {
            return Ok(());
        }
        if Self::has_permission(&user.id, permission).await? {
            Ok(())
        } else {
            bail!("forbidden")
        }
    }
}
