use dioxus::prelude::*;

#[component]
pub fn Headline(
    #[props(extends=GlobalAttributes)]
    #[props(extends=h2)]
    attributes: Vec<Attribute>,
    children: Element,
    #[props(into, default)] class: String,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        h2 {
            class: format!("headline {}", class),
            ..attributes,
            {children}
        }
    }
}
