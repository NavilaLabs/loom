use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use ui::{
    components::atoms::{Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarTrigger},
    views::{Database, Login},
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
}

fn main() {
    dioxus::launch(App);
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
