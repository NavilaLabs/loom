use dioxus::prelude::*;

use crate::components::molecules::ThemeSwitcher;
use crate::components::organisms::Header;

#[component]
pub fn DefaultLayout(#[props(into)] headline: Option<Element>, children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/theme.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "default-layout w-full px-4",
            div { class: "default-layout-headline", {headline} },
            div { class: "default-layout-content w-full sm:pt-2 md:pt-4 lg:pt-8 self-center", {children} }
        }
    }
}
