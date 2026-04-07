use anyhow::Result;
use eventually::aggregate::repository::Saver;
use loom_core::admin::{
    user::{UserEvent, UserId},
    workspace::{WorkspaceEvent, WorkspaceId},
};
use loom_infrastructure_impl::{
    Pool,
    admin::{
        authentication::hash_password,
        user::repositories::UserRepository,
        workspace::repositories::WorkspaceRepository,
    },
};

pub async fn setup_application(
    username: String,
    email: String,
    password: String,
    workspace_name: String,
) -> Result<()> {
    let pool = Pool::connect_admin().await?;

    let user_repo = UserRepository::from_pool(pool.clone()).await?;

    if user_repo.has_at_least_one_user().await? {
        anyhow::bail!("application is already set up");
    }

    let password_hash = hash_password(&password)?;

    let user_id = UserId::new();
    let mut user_root = eventually::aggregate::Root::<loom_core::admin::user::User>::record_new(
        UserEvent::Created {
            id: user_id,
            name: username,
            email,
            password_hash,
        }
        .into(),
    )?;
    user_repo.save(&mut user_root).await?;

    let workspace_repo = WorkspaceRepository::from_pool(pool).await?;
    let workspace_id = WorkspaceId::new();
    let mut workspace_root =
        eventually::aggregate::Root::<loom_core::admin::workspace::Workspace>::record_new(
            WorkspaceEvent::Created {
                id: workspace_id,
                name: Some(workspace_name),
            }
            .into(),
        )?;
    workspace_repo.save(&mut workspace_root).await?;

    Ok(())
}
