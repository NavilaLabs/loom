use dioxus::prelude::*;

use ui::views::Login;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(DefaultLayout)]
    #[route("/login")]
    Login {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }

        Router::<Route> {}
    }
}

// A web-specific Router around the shared `Navbar` component
// which allows us to use the web-specific `Route` enum.
#[component]
fn DefaultLayout() -> Element {
    rsx! {
        // document::Link { rel: "stylesheet", href: asset!("/../../ui/assets/theme.css") }

        Outlet::<Route> {}
    }
}
