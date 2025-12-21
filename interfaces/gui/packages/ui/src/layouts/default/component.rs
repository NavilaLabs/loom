use dioxus::prelude::*;

#[component]
pub fn DefaultLayout(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/theme.css") }
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "default-layout", {children} }
    }
}
