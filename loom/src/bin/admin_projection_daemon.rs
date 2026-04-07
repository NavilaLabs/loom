use anyhow::Result;
use loom::infrastructure::{
    BackoffConfig, Pool, ProjectionDaemon, ProjectionRunner, ProjectionSource, SqlCheckpoint,
    admin::{
        user::projectors::UserProjector,
        workspace::projectors::WorkspaceProjector,
        workspace_role::projectors::WorkspaceRoleProjector,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    dotenvy::from_filename_override(".env.dev").ok();

    let pool = Pool::connect_admin().await?;

    // Add the global_position column + trigger that the projection runner needs.
    // This is idempotent — safe to call on every startup.
    ProjectionRunner::new(pool.clone().into_pool(), ProjectionSource::AllStreams)
        .run_migrations()
        .await?;

    let backoff = BackoffConfig {
        min_idle_ms: 20,
        max_idle_ms: 200,
        ..Default::default()
    };

    let mut daemon = ProjectionDaemon::new();

    daemon.register_with_config(
        ProjectionRunner::new(pool.clone().into_pool(), ProjectionSource::AllStreams),
        UserProjector::new(pool.clone()),
        SqlCheckpoint::new(pool.clone().into_pool(), "user_projection").await?,
        backoff.clone(),
    );

    daemon.register_with_config(
        ProjectionRunner::new(pool.clone().into_pool(), ProjectionSource::AllStreams),
        WorkspaceProjector::new(pool.clone()),
        SqlCheckpoint::new(pool.clone().into_pool(), "workspace_projection").await?,
        backoff.clone(),
    );

    daemon.register_with_config(
        ProjectionRunner::new(pool.clone().into_pool(), ProjectionSource::AllStreams),
        WorkspaceRoleProjector::new(pool.clone()),
        SqlCheckpoint::new(pool.clone().into_pool(), "workspace_role_projection").await?,
        backoff,
    );

    daemon.run_until_cancelled().await;

    Ok(())
}
