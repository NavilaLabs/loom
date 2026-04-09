use dioxus::prelude::*;

use crate::components::molecules::SettingsMenu;

#[component]
pub fn Header(
    /// Current page title shown on the left of the header bar.
    /// Pass an empty string to show only the actions area (e.g. on login/setup).
    #[props(default)]
    title: String,
) -> Element {
    rsx! {
        // Load global stylesheets here (in addition to DefaultLayout) so they
        // are never removed from the document head during route transitions.
        document::Link { rel: "stylesheet", href: asset!("/assets/theme.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        header { class: "header",
            div { class: "header-content",
                if !title.is_empty() {
                    h1 { class: "header-title", "{title}" }
                }
                div { class: "header-actions",
                    SettingsMenu {}
                }
            }
        }
    }
}
