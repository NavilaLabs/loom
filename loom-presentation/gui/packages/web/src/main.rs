use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;
use ui::{
    components::{atoms::Headline, molecules::ThemeSwitcher, organisms::Header},
    views::{setup::Setup, Dashboard, Database, Login},
    FAVICON,
};

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
        #[route("/login")]
        Login {},
        #[route("/dashboard")]
        Dashboard {},
        #[route("/developer/database")]
        Database {},
        #[route("/setup")]
        Setup {},
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
    let resolver: TransitionVariantResolver<Route> = std::rc::Rc::new(|from, to| {
        fn idx(route: &Route) -> i32 {
            match route {
                Route::Login { .. } => 0,
                Route::Setup { .. } => 1,
                Route::Database { .. } => 2,
                Route::Dashboard { .. } => 3,
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

    // To use a Tween for page transitions, provide it via context:
    let tween = use_signal(|| Tween {
        duration: std::time::Duration::from_millis(500),
        easing: easer::functions::Cubic::ease_in_out,
    });
    use_context_provider(|| tween);

    resource_pools::init_high_performance();

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
fn Layout() -> Element {
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

    dbg!(route.to_string());

    rsx! {
        Header {}

        Headline { { convert_case::ccase!(title, last_path_part) } }

        AnimatedOutlet::<Route> {}

        ThemeSwitcher {}
    }
}
