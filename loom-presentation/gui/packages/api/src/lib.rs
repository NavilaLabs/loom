//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;
#[cfg(feature = "server")]
use loom::infrastructure::{
    admin::user::projectors::UserProjector, BackoffConfig, Pool, ProjectionDaemon,
    ProjectionRunner, ProjectionSource, SqlCheckpoint,
};

pub mod developer;

#[cfg(feature = "server")]
pub async fn configure_admin_projection_daemon() -> Result<Option<ProjectionDaemon>> {
    let pool = Pool::connect_admin().await?;

    let runner = ProjectionRunner::new(pool.clone().into_pool(), ProjectionSource::AllStreams);

    let user_projector = UserProjector::new(pool.clone());

    let mut daemon = ProjectionDaemon::new();
    daemon.register_with_config(
        runner,
        user_projector,
        SqlCheckpoint::new(pool.clone().into_pool(), "user_projection").await?,
        BackoffConfig {
            min_idle_ms: 20,
            max_idle_ms: 200,
            ..Default::default()
        },
    );

    Ok(Some(daemon))
}
