pub mod activity;
pub mod activity_rate;
pub mod customer;
pub mod project;
pub mod project_rate;
pub mod tag;
pub mod timesheet;
pub mod user;

/// Open a connection pool to the given workspace's tenant database.
///
/// Extracted here to avoid repeating the identical boilerplate in every
/// controller module (customer, project, activity, tag, timesheet, …).
pub(super) async fn tenant_pool(
    workspace_id: &str,
) -> anyhow::Result<loom_infrastructure_impl::ConnectedTenantPool> {
    Ok(loom_infrastructure_impl::Pool::<
        loom_infrastructure_impl::ScopeTenant,
        loom_infrastructure_impl::StateDisconnected,
    >::connect_tenant(workspace_id)
    .await?)
}
