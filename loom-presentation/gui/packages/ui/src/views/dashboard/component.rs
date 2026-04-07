use crate::layouts::DefaultLayout;
use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        DefaultLayout {
            p { class: "text-muted-foreground", "Welcome to Loom." }
        }
    }
}
