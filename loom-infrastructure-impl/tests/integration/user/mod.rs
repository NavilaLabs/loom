use eventually::aggregate::{
    Aggregate,
    repository::{Getter, Saver},
};
use eventually_any::snapshot::Repository;
use eventually_projection::{Projector, RawEvent};
use loom_core::admin::user::{User, UserEvent, UserId};
use loom_infrastructure_impl::{
    Pool, ScopeAdmin, StateConnected, admin::user::projectors::UserProjector,
};
use sqlx::Row;
use with_lifecycle::with_lifecycle;

use crate::database::{get_admin_pool, get_default_pool, refresh_databases, test_lifecycle};

// ── helpers ───────────────────────────────────────────────────────────────────

fn test_id() -> UserId {
    "019d0ce8-facb-7c90-b9d7-287ae4f17c91"
        .parse()
        .expect("valid UUID")
}

type UserRepo = Repository<User, eventually::serde::Json<User>, eventually::serde::Json<UserEvent>>;

async fn make_repository(
    pool: &Pool<ScopeAdmin, StateConnected>,
) -> Result<UserRepo, sqlx::migrate::MigrateError> {
    Repository::new(
        pool.as_ref().clone(),
        eventually::serde::Json::default(),
        eventually::serde::Json::default(),
    )
    .await
}

// ── tests ─────────────────────────────────────────────────────────────────────

pub mod tests {
    use serial_test::serial;

    use super::*;

    /// Saving a new aggregate root persists it; loading it back returns the
    /// same state and version.
    #[serial]
    #[with_lifecycle(test_lifecycle)]
    #[tokio::test]
    async fn test_save_and_get_user() -> Result<(), Box<dyn std::error::Error>> {
        let default_pool = get_default_pool().await?;
        refresh_databases(&default_pool, "test_token").await?;

        let admin_pool = get_admin_pool().await?;
        let repo = make_repository(&admin_pool).await?;
        let id = test_id();

        let mut root = eventually::aggregate::Root::<User>::record_new(
            UserEvent::Created {
                id: id.clone(),
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
                password_hash: "".to_string(),
            }
            .into(),
        )
        .expect("Created event on a new aggregate is always valid");

        repo.save(&mut root).await?;

        let loaded = repo.get(&id).await?;
        assert_eq!(loaded.aggregate_id(), &id);
        assert_eq!(loaded.name(), "Alice");
        assert_eq!(loaded.version(), 1);

        Ok(())
    }

    /// Applying a second `Created` event to an already-existing `User` must
    /// return an `AlreadyExists` error — pure domain logic, no database needed.
    #[test]
    fn test_duplicate_user_creation_is_rejected_by_domain() {
        let id = test_id();
        let existing = User::apply(
            None,
            UserEvent::Created {
                id: id.clone(),
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
                password_hash: "".to_string(),
            },
        )
        .expect("first Created is valid");

        let result = User::apply(
            Some(existing),
            UserEvent::Created {
                id,
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
                password_hash: "".to_string(),
            },
        );
        assert!(
            result.is_err(),
            "second Created on an existing user must fail"
        );
    }

    /// The projector must insert a row into the projection table when it
    /// receives a `UserCreated` event.
    #[serial]
    #[with_lifecycle(test_lifecycle)]
    #[tokio::test]
    async fn test_projector_inserts_row_on_user_created() -> Result<(), Box<dyn std::error::Error>>
    {
        let default_pool = get_default_pool().await?;
        refresh_databases(&default_pool, "test_token").await?;

        let admin_pool = get_admin_pool().await?;
        let mut projector = UserProjector::new(admin_pool.clone());

        let id = test_id();
        let event = UserEvent::Created {
            id: id.clone(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            password_hash: "".to_string(),
        };
        let payload_bytes = serde_json::to_vec(&event)?;

        projector
            .handle(RawEvent {
                stream_id: id.to_string(),
                version: 1,
                global_position: 1,
                event_type: "UserCreated".to_string(),
                payload_bytes,
                metadata: serde_json::Value::Null,
                schema_version: 1,
            })
            .await?;

        let rows = sqlx::query("SELECT name FROM projections__users")
            .fetch_all(admin_pool.as_ref())
            .await?;

        let found = rows.iter().any(|r| {
            let name: String = r.get("name");
            name == "Alice"
        });
        assert!(found, "projection table should contain a row for Alice");

        Ok(())
    }

    /// The projector must silently ignore event types it does not handle.
    #[serial]
    #[with_lifecycle(test_lifecycle)]
    #[tokio::test]
    async fn test_projector_ignores_unknown_event_type() -> Result<(), Box<dyn std::error::Error>> {
        let default_pool = get_default_pool().await?;
        refresh_databases(&default_pool, "test_token").await?;

        let admin_pool = get_admin_pool().await?;
        let mut projector = UserProjector::new(admin_pool);

        let result = projector
            .handle(RawEvent {
                stream_id: "stream-1".to_string(),
                version: 1,
                global_position: 1,
                event_type: "UnknownEvent".to_string(),
                payload_bytes: b"{}".to_vec(),
                metadata: serde_json::Value::Null,
                schema_version: 1,
            })
            .await;

        assert!(
            result.is_ok(),
            "unknown event type must not produce an error"
        );

        Ok(())
    }
}
