use anyhow::Result;
use loom::infrastructure::{
    BackoffConfig, Pool, ProjectionDaemon, ProjectionRunner, ProjectionSource, SqlCheckpoint,
    admin::projectors::AdminProjector,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

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

    // All admin projectors run under a single runner with a single checkpoint.
    // This guarantees events are applied sequentially across all projection
    // tables, preventing FK race conditions between independent runners.
    daemon.register_with_config(
        ProjectionRunner::new(pool.clone().into_pool(), ProjectionSource::AllStreams),
        AdminProjector::new(pool.clone()),
        SqlCheckpoint::new(pool.clone().into_pool(), "admin_projection").await?,
        backoff,
    );

    daemon.run_until_cancelled().await;

    Ok(())
}
