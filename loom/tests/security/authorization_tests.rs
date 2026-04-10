/// Integration tests for `AuthorizationService`.
///
/// Each test resets the database to a clean state, seeds the projection
/// tables with known data, then calls the service under test.  The service
/// always opens its own pool via `Pool::connect_admin()`, so it picks up
/// whatever the test database contains — no mocking needed.
///
/// Security scenarios covered:
///   - Admin flag correctly detected via role name                ✓
///   - Non-admin roles do NOT grant admin                         ✓
///   - Unknown users are never treated as admin                   ✓
///   - Empty / zero-length user_id is safe                        ✓
///   - SQL injection in user_id returns false (parameterised)     ✓
///   - SQL injection in permission name returns false             ✓
///   - Permission via role grant                                  ✓
///   - Permission via direct user grant                           ✓
///   - Absent permission returns false                            ✓
///   - Role name "admin" is case-sensitive                        ✓
///   - `require_admin` returns Ok for admins, Err for others      ✓
///   - `require_permission` admin bypass works                    ✓
///   - `require_permission` returns Err when permission absent    ✓
use loom::{auth::CurrentUser, authorization::AuthorizationService};
use loom_tests::{get_admin_pool, get_default_pool, refresh_databases, test_database_lifecycle};
use serial_test::serial;
use with_lifecycle::with_lifecycle;

// ── Fixed test identifiers ────────────────────────────────────────────────────
// Using deterministic UUIDs so tests are reproducible and readable.

const WORKSPACE_ID: &str = "00000000-0000-0000-0000-000000000001";
const ADMIN_USER_ID: &str = "00000000-0000-0000-0000-000000000010";
const REGULAR_USER_ID: &str = "00000000-0000-0000-0000-000000000011";
const UNRELATED_USER_ID: &str = "00000000-0000-0000-0000-000000000012";
const ADMIN_ROLE_ID: &str = "00000000-0000-0000-0000-000000000020";
const VIEWER_ROLE_ID: &str = "00000000-0000-0000-0000-000000000021";
const PERMISSION_ID: &str = "00000000-0000-0000-0000-000000000030";
const PERMISSION_NAME: &str = "workspace.read";

// ── Helpers ───────────────────────────────────────────────────────────────────

fn admin_user() -> CurrentUser {
    CurrentUser {
        id: ADMIN_USER_ID.to_string(),
        email: "admin@test.com".to_string(),
    }
}

fn regular_user() -> CurrentUser {
    CurrentUser {
        id: REGULAR_USER_ID.to_string(),
        email: "user@test.com".to_string(),
    }
}

fn unknown_user() -> CurrentUser {
    CurrentUser {
        id: "00000000-0000-0000-0000-999999999999".to_string(),
        email: "ghost@test.com".to_string(),
    }
}

/// Resets and re-migrates the test databases, then seeds the projection
/// tables with a known baseline state for all authorization tests.
///
/// Seed state:
///  - One workspace
///  - Two users: admin_user (has "admin" role), regular_user (has "viewer" role)
///  - Two roles: "admin", "viewer"
///  - One permission: PERMISSION_NAME, granted to "viewer" role AND directly to regular_user
async fn reset_and_seed() -> Result<(), Box<dyn std::error::Error>> {
    let default_pool = get_default_pool().await?;
    refresh_databases(&default_pool, "security_test").await?;

    let admin_pool = get_admin_pool().await?;
    let pool = admin_pool.as_ref();

    // 1. Workspace (required by FK chain for workspace_roles and workspace_user_roles)
    sqlx::query("INSERT INTO projections__workspaces (id, name) VALUES ($1, $2)")
        .bind(WORKSPACE_ID)
        .bind("Test Workspace")
        .execute(pool)
        .await?;

    // 2. Users (required by FK from workspace_user_roles and workspace_user_permissions)
    sqlx::query(
        "INSERT INTO projections__users (id, name, email, password) VALUES ($1, $2, $3, $4)",
    )
    .bind(ADMIN_USER_ID)
    .bind("Admin User")
    .bind("admin@test.com")
    .bind("$2b$12$placeholder_hash")
    .execute(pool)
    .await?;

    sqlx::query(
        "INSERT INTO projections__users (id, name, email, password) VALUES ($1, $2, $3, $4)",
    )
    .bind(REGULAR_USER_ID)
    .bind("Regular User")
    .bind("user@test.com")
    .bind("$2b$12$placeholder_hash")
    .execute(pool)
    .await?;

    // 3. Permissions (no FK deps)
    sqlx::query("INSERT INTO permissions (id, name) VALUES ($1, $2)")
        .bind(PERMISSION_ID)
        .bind(PERMISSION_NAME)
        .execute(pool)
        .await?;

    // 4. Workspace roles (FK → workspaces)
    sqlx::query(
        "INSERT INTO projections__workspace_roles (id, workspace_id, name) VALUES ($1, $2, $3)",
    )
    .bind(ADMIN_ROLE_ID)
    .bind(WORKSPACE_ID)
    .bind("admin")
    .execute(pool)
    .await?;

    sqlx::query(
        "INSERT INTO projections__workspace_roles (id, workspace_id, name) VALUES ($1, $2, $3)",
    )
    .bind(VIEWER_ROLE_ID)
    .bind(WORKSPACE_ID)
    .bind("viewer")
    .execute(pool)
    .await?;

    // 5. User ↔ role assignments (FK → workspaces, users, workspace_roles)
    sqlx::query(
        "INSERT INTO projections__workspace_user_roles (workspace_id, user_id, workspace_role_id)
         VALUES ($1, $2, $3)",
    )
    .bind(WORKSPACE_ID)
    .bind(ADMIN_USER_ID)
    .bind(ADMIN_ROLE_ID)
    .execute(pool)
    .await?;

    sqlx::query(
        "INSERT INTO projections__workspace_user_roles (workspace_id, user_id, workspace_role_id)
         VALUES ($1, $2, $3)",
    )
    .bind(WORKSPACE_ID)
    .bind(REGULAR_USER_ID)
    .bind(VIEWER_ROLE_ID)
    .execute(pool)
    .await?;

    // 6. Role ↔ permission grants (FK → workspace_roles, permissions)
    sqlx::query(
        "INSERT INTO projections__workspace_role_permissions (workspace_role_id, permission_id)
         VALUES ($1, $2)",
    )
    .bind(VIEWER_ROLE_ID)
    .bind(PERMISSION_ID)
    .execute(pool)
    .await?;

    // 7. Direct user ↔ permission grant for regular_user (FK → workspaces, users, permissions)
    sqlx::query(
        "INSERT INTO projections__workspace_user_permissions (workspace_id, user_id, permission_id)
         VALUES ($1, $2, $3)",
    )
    .bind(WORKSPACE_ID)
    .bind(REGULAR_USER_ID)
    .bind(PERMISSION_ID)
    .execute(pool)
    .await?;

    Ok(())
}

// ── is_admin ──────────────────────────────────────────────────────────────────

#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn is_admin_returns_true_for_user_with_admin_role() {
    reset_and_seed().await.unwrap();
    assert!(AuthorizationService::is_admin(ADMIN_USER_ID).await.unwrap());
}

#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn is_admin_returns_false_for_user_with_non_admin_role() {
    reset_and_seed().await.unwrap();
    assert!(
        !AuthorizationService::is_admin(REGULAR_USER_ID)
            .await
            .unwrap()
    );
}

#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn is_admin_returns_false_for_unknown_user() {
    reset_and_seed().await.unwrap();
    assert!(
        !AuthorizationService::is_admin(UNRELATED_USER_ID)
            .await
            .unwrap(),
        "a user that exists nowhere in the system must not be admin"
    );
}

#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn is_admin_returns_false_for_empty_user_id() {
    reset_and_seed().await.unwrap();
    assert!(
        !AuthorizationService::is_admin("").await.unwrap(),
        "empty user_id must not match any admin role"
    );
}

/// SQL injection via user_id: the query must use parameterised binds so that
/// an injected string is treated as a literal value, not SQL syntax.
/// The injected input must return false, not grant access.
#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn is_admin_sql_injection_in_user_id_is_neutralised() {
    reset_and_seed().await.unwrap();

    let injections = [
        "' OR '1'='1",
        "' OR 1=1--",
        "'; DROP TABLE projections__workspace_user_roles;--",
        "\"; SELECT * FROM projections__workspace_roles WHERE name='admin'--",
        "\\x00",
    ];

    for injection in &injections {
        let result = AuthorizationService::is_admin(injection).await.unwrap();
        assert!(
            !result,
            "SQL injection attempt {:?} must not grant admin access",
            injection
        );
    }
}

/// Role name matching is case-sensitive: "Admin" must not satisfy the check
/// for "admin".  This prevents privilege escalation by creating a role named
/// "Admin" or "ADMIN".
#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn is_admin_role_name_matching_is_case_sensitive() {
    reset_and_seed().await.unwrap();
    let pool = get_admin_pool().await.unwrap();

    // Insert a role named "Admin" (capital A) assigned to UNRELATED_USER_ID.
    let capital_role_id = "00000000-0000-0000-0000-000000000099";
    sqlx::query(
        "INSERT INTO projections__workspace_roles (id, workspace_id, name) VALUES ($1, $2, $3)",
    )
    .bind(capital_role_id)
    .bind(WORKSPACE_ID)
    .bind("Admin") // capital A — should NOT match "admin" check
    .execute(pool.as_ref())
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO projections__users (id, name, email, password) VALUES ($1, $2, $3, $4)",
    )
    .bind(UNRELATED_USER_ID)
    .bind("Unrelated")
    .bind("unrelated@test.com")
    .bind("$2b$12$placeholder")
    .execute(pool.as_ref())
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO projections__workspace_user_roles (workspace_id, user_id, workspace_role_id)
         VALUES ($1, $2, $3)",
    )
    .bind(WORKSPACE_ID)
    .bind(UNRELATED_USER_ID)
    .bind(capital_role_id)
    .execute(pool.as_ref())
    .await
    .unwrap();

    assert!(
        !AuthorizationService::is_admin(UNRELATED_USER_ID)
            .await
            .unwrap(),
        "role named 'Admin' (capital A) must not satisfy the 'admin' check"
    );
}

// ── has_permission ────────────────────────────────────────────────────────────

#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn has_permission_returns_true_via_role_grant() {
    reset_and_seed().await.unwrap();
    // regular_user has viewer role, viewer role has PERMISSION_NAME via role grant
    assert!(
        AuthorizationService::has_permission(REGULAR_USER_ID, PERMISSION_NAME)
            .await
            .unwrap()
    );
}

#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn has_permission_returns_true_via_direct_user_grant() {
    reset_and_seed().await.unwrap();
    // regular_user also has PERMISSION_NAME granted directly in workspace_user_permissions
    // Verify the direct-grant path by using a user that has NO role with the permission
    // but does have a direct grant.
    let direct_only_user_id = "00000000-0000-0000-0000-000000000050";
    let pool = get_admin_pool().await.unwrap();

    sqlx::query(
        "INSERT INTO projections__users (id, name, email, password) VALUES ($1, $2, $3, $4)",
    )
    .bind(direct_only_user_id)
    .bind("Direct Grant User")
    .bind("direct@test.com")
    .bind("$2b$12$placeholder")
    .execute(pool.as_ref())
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO projections__workspace_user_permissions (workspace_id, user_id, permission_id)
         VALUES ($1, $2, $3)",
    )
    .bind(WORKSPACE_ID)
    .bind(direct_only_user_id)
    .bind(PERMISSION_ID)
    .execute(pool.as_ref())
    .await
    .unwrap();

    assert!(
        AuthorizationService::has_permission(direct_only_user_id, PERMISSION_NAME)
            .await
            .unwrap(),
        "direct permission grant must be sufficient without a role"
    );
}

#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn has_permission_returns_false_for_user_with_no_grant() {
    reset_and_seed().await.unwrap();
    // ADMIN_USER_ID has the admin role, but that role has no permissions assigned.
    assert!(
        !AuthorizationService::has_permission(ADMIN_USER_ID, PERMISSION_NAME)
            .await
            .unwrap(),
        "admin role has no permission grants in this test — must return false"
    );
}

#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn has_permission_returns_false_for_unknown_permission_name() {
    reset_and_seed().await.unwrap();
    assert!(
        !AuthorizationService::has_permission(REGULAR_USER_ID, "permission.that.does.not.exist")
            .await
            .unwrap()
    );
}

#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn has_permission_returns_false_for_unknown_user() {
    reset_and_seed().await.unwrap();
    assert!(
        !AuthorizationService::has_permission(UNRELATED_USER_ID, PERMISSION_NAME)
            .await
            .unwrap()
    );
}

/// SQL injection via the permission name parameter must not grant access.
#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn has_permission_sql_injection_in_permission_name_is_neutralised() {
    reset_and_seed().await.unwrap();

    let injections = [
        "' OR '1'='1",
        "' OR 1=1--",
        "workspace.read' OR 'x'='x",
        "'; DROP TABLE permissions;--",
    ];

    for injection in &injections {
        let result = AuthorizationService::has_permission(ADMIN_USER_ID, injection)
            .await
            .unwrap();
        assert!(
            !result,
            "SQL injection in permission name {:?} must not grant access",
            injection
        );
    }
}

// ── require_admin ─────────────────────────────────────────────────────────────

#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn require_admin_succeeds_for_admin_user() {
    reset_and_seed().await.unwrap();
    AuthorizationService::require_admin(&admin_user())
        .await
        .expect("require_admin must return Ok for the admin user");
}

#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn require_admin_fails_for_regular_user() {
    reset_and_seed().await.unwrap();
    let err = AuthorizationService::require_admin(&regular_user())
        .await
        .expect_err("require_admin must fail for a non-admin user");
    assert!(
        err.to_string().contains("forbidden"),
        "error message must say 'forbidden', got: {err}"
    );
}

#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn require_admin_fails_for_unknown_user() {
    reset_and_seed().await.unwrap();
    let err = AuthorizationService::require_admin(&unknown_user())
        .await
        .expect_err("require_admin must fail for a user not in the system");
    assert!(err.to_string().contains("forbidden"));
}

// ── require_permission ────────────────────────────────────────────────────────

/// Admin users bypass individual permission checks — they can do everything.
/// This test verifies the short-circuit: the admin_user has NO explicit
/// permission grants, yet `require_permission` passes.
#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn require_permission_admin_bypasses_permission_check() {
    reset_and_seed().await.unwrap();
    AuthorizationService::require_permission(&admin_user(), PERMISSION_NAME)
        .await
        .expect("admin must bypass permission checks even without an explicit grant");
}

#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn require_permission_succeeds_for_user_with_permission() {
    reset_and_seed().await.unwrap();
    AuthorizationService::require_permission(&regular_user(), PERMISSION_NAME)
        .await
        .expect("user with the permission must pass require_permission");
}

#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn require_permission_fails_for_user_without_permission() {
    reset_and_seed().await.unwrap();
    let err = AuthorizationService::require_permission(&regular_user(), "permission.not.granted")
        .await
        .expect_err("user without the permission must fail require_permission");
    assert!(
        err.to_string().contains("forbidden"),
        "error must say 'forbidden', got: {err}"
    );
}

#[serial]
#[with_lifecycle(test_database_lifecycle)]
#[tokio::test]
async fn require_permission_fails_for_unknown_user() {
    reset_and_seed().await.unwrap();
    let err = AuthorizationService::require_permission(&unknown_user(), PERMISSION_NAME)
        .await
        .expect_err("unknown user must fail require_permission");
    assert!(err.to_string().contains("forbidden"));
}
