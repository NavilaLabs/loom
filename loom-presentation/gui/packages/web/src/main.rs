use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use ui::{
    views::{setup::Admin as SetupAdmin, Database, Login},
    FAVICON,
};

#[derive(Debug, Clone, Routable, PartialEq, MotionTransitions)]
#[rustfmt::skip]
enum Route {
    #[layout(DefaultLayout)]
        #[route("/login")]
        #[transition()]
        Login {},
        #[route("/developer/database")]
        #[transition()]
        Database {},
        #[route("/setup/admin")]
        #[transition()]
        SetupAdmin {},
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

    let projection_daemon = api::configure_admin_projection_daemon()
        .await
        .unwrap()
        .unwrap();

    tokio::spawn(async move {
        projection_daemon.run_until_cancelled().await;
    });

    let address = dioxus::cli_config::fullstack_address_or_localhost();

    let router =
        axum::Router::new().serve_dioxus_application(dioxus_server::ServeConfig::new(), App);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Title { "Loom" }
        document::Meta { name: "description", content: "Loom" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Link { rel: "icon", href: FAVICON }

        Router::<Route> {}
    }
}

// A web-specific Router around the shared `Navbar` component
// which allows us to use the web-specific `Route` enum.
#[component]
fn DefaultLayout() -> Element {
    rsx! {
        AnimatedOutlet::<Route> {}
    }
}
