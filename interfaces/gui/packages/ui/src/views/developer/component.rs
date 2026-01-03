use crate::components::atoms::{Button, Label};
use crate::layouts::DefaultLayout;
use dioxus::prelude::*;

#[component]
pub fn Developer() -> Element {
    rsx! {
        DefaultLayout {
            h2 { "Developer" }

            div { class: "grid gap-4 md:grid-cols-2 md:gap-6",
                div {
                    Label { html_for: "run-migrations-btn", "Migrations" }
                    Button { id: "run-migrations-btn", "Run Migrations" }
                }
            }
        }
    }
}
