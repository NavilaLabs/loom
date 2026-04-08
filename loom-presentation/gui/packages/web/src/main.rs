use api::auth::UserInfo;
use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;
use ui::{
    components::{atoms::Headline, molecules::ThemeSwitcher, organisms::Header},
    views::{
        setup::Setup, Activities, Customers, Dashboard, Database, Login, Projects, SelectWorkspace,
        Timesheets,
    },
    FAVICON,
};

/// Three-state auth signal shared across the whole app.
///
/// - `None`           → still checking (initial page load)
/// - `Some(None)`     → confirmed not authenticated
/// - `Some(Some(u))`  → confirmed authenticated as `u`
pub type AuthState = Signal<Option<Option<UserInfo>>>;

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "404 - Page Not Found" }
    }
}

#[derive(Debug, Clone, Routable, PartialEq, MotionTransitions)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]

        // Setup — only accessible when setup is NOT yet complete.
        #[layout(RequireSetupIncomplete)]
            #[route("/setup")]
            Setup {},
        #[end_layout]

        // All remaining routes — only accessible after setup IS complete.
        #[layout(RequireSetupComplete)]
            #[route("/login")]
            Login {},

            #[layout(RequireAuth)]
                // Workspace selection — accessible to any authenticated user.
                #[route("/select-workspace")]
                SelectWorkspace {},

                // All routes below additionally require a workspace to be selected.
                #[layout(RequireWorkspace)]
                    #[route("/dashboard")]
                    Dashboard {},

                    #[route("/customers")]
                    Customers {},

                    #[route("/projects")]
                    Projects {},

                    #[route("/activities")]
                    Activities {},

                    #[route("/timesheets")]
                    Timesheets {},

                    #[layout(RequireAdmin)]
                        #[route("/developer/database")]
                        Database {},
                    #[end_layout]
                #[end_layout]
            #[end_layout]
        #[end_layout]

    #[end_layout]

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[cfg(not(feature = "server"))]
fn main() {
    dotenvy::from_filename_override(".env.dev").ok();
    dioxus::launch(App);
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    dotenvy::from_filename_override(".env.dev").ok();

    // Ensure the admin database exists and is fully migrated before serving.
    loom::setup::init_admin_db()
        .await
        .expect("failed to initialise admin database");

    let address = dioxus::cli_config::fullstack_address_or_localhost();

    let session_store = tower_sessions::MemoryStore::default();
    let session_layer = tower_sessions::SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(tower_sessions::cookie::SameSite::Lax);

    let router = axum::Router::new()
        .serve_dioxus_application(dioxus_server::ServeConfig::new(), App)
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

#[component]
fn App() -> Element {
    // Global auth state — available to every component in the tree.
    use_context_provider(|| Signal::new(None::<Option<UserInfo>>));

    let resolver: TransitionVariantResolver<Route> = std::rc::Rc::new(|from, to| {
        fn idx(route: &Route) -> i32 {
            match route {
                Route::Login { .. } => 0,
                Route::Setup { .. } => 1,
                Route::SelectWorkspace { .. } => 2,
                Route::Dashboard { .. } => 3,
                Route::Customers { .. } => 4,
                Route::Projects { .. } => 5,
                Route::Activities { .. } => 6,
                Route::Timesheets { .. } => 7,
                Route::Database { .. } => 8,
                _ => -1,
            }
        }
        let from_idx = idx(from);
        let to_idx = idx(to);
        if from_idx != -1 && to_idx != -1 {
            if to_idx > from_idx {
                TransitionVariant::SlideLeft
            } else if to_idx < from_idx {
                TransitionVariant::SlideRight
            } else {
                TransitionVariant::Fade
            }
        } else {
            to.get_transition()
        }
    });
    use_context_provider(|| resolver);

    let tween = use_signal(|| Tween {
        duration: std::time::Duration::from_millis(500),
        easing: easer::functions::Cubic::ease_in_out,
    });
    use_context_provider(|| tween);

    rsx! {
        document::Title { "Loom" }
        document::Meta { name: "description", content: "Loom" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Link { rel: "icon", href: FAVICON }

        Router::<Route> {}
    }
}

// ── Top-level layout ──────────────────────────────────────────────────────────

#[component]
fn Layout() -> Element {
    let mut auth: AuthState = use_context();

    // Fetch session auth state once when the layout mounts.
    use_resource(move || async move {
        let user = api::auth::get_current_user().await.ok().flatten();
        auth.set(Some(user));
    });

    let route = use_route::<Route>();
    let route_parts: Vec<String> = route
        .to_string()
        .split("/")
        .map(|part| part.to_string())
        .collect();
    let last_path_part: String = route_parts[route_parts.len() - 1]
        .split("?")
        .map(|path| path.to_string())
        .collect::<Vec<String>>()[0]
        .clone();

    rsx! {
        Header {}
        Headline { { convert_case::ccase!(title, last_path_part) } }
        AnimatedOutlet::<Route> {}
        ThemeSwitcher {}
    }
}

// ── Route guards ──────────────────────────────────────────────────────────────

/// Redirects to /setup when setup is NOT complete; renders children otherwise.
#[component]
fn RequireSetupComplete() -> Element {
    let nav = use_navigator();
    let complete = use_resource(|| async { api::setup::is_setup_complete().await });

    match complete.value().cloned() {
        None => rsx! {},
        Some(Ok(true)) => rsx! { Outlet::<Route> {} },
        Some(Ok(false)) | Some(Err(_)) => {
            nav.replace(Route::Setup {});
            rsx! {}
        }
    }
}

/// Redirects to /login when setup IS complete; renders children otherwise.
#[component]
fn RequireSetupIncomplete() -> Element {
    let nav = use_navigator();
    let complete = use_resource(|| async { api::setup::is_setup_complete().await });

    match complete.value().cloned() {
        None => rsx! {},
        Some(Err(_)) | Some(Ok(false)) => rsx! { Outlet::<Route> {} },
        Some(Ok(true)) => {
            nav.replace(Route::Login {});
            rsx! {}
        }
    }
}

/// Reads global auth state. Shows nothing while loading, redirects to /login
/// when unauthenticated, and renders the outlet when authenticated.
#[component]
fn RequireAuth() -> Element {
    let nav = use_navigator();
    let auth: AuthState = use_context();

    match auth.cloned() {
        None => rsx! {},
        Some(None) => {
            nav.replace(Route::Login {});
            rsx! {}
        }
        Some(Some(_)) => rsx! { AnimatedOutlet::<Route> {} },
    }
}

/// Redirects to /select-workspace when the authenticated user has not yet
/// chosen a workspace for this session.
#[component]
fn RequireWorkspace() -> Element {
    let nav = use_navigator();
    let auth: AuthState = use_context();

    match auth.cloned() {
        Some(Some(user)) if user.workspace_id.is_some() => {
            rsx! { AnimatedOutlet::<Route> {} }
        }
        Some(Some(_)) => {
            // Authenticated but no workspace selected yet.
            nav.replace(Route::SelectWorkspace {});
            rsx! {}
        }
        // Loading or unauthenticated — RequireAuth above handles these.
        _ => rsx! {},
    }
}

/// Redirects to /dashboard when the authenticated user is not an admin.
#[component]
fn RequireAdmin() -> Element {
    let nav = use_navigator();
    let auth: AuthState = use_context();

    match auth.cloned() {
        Some(Some(user)) if user.is_admin => rsx! { Outlet::<Route> {} },
        Some(Some(_)) => {
            nav.replace(Route::Dashboard {});
            rsx! {}
        }
        // Loading or unauthenticated — RequireAuth handles these cases above us.
        _ => rsx! {},
    }
}
