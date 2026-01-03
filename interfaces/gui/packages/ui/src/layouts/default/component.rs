use dioxus::prelude::*;

use crate::components::organisms::Header;

#[component]
pub fn DefaultLayout(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/theme.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        Header {}

        div { class: "default-layout", {children} }
    }
}
