use dioxus::prelude::*;

#[component]
pub fn Headline(
    #[props(extends=GlobalAttributes)]
    #[props(extends=h2)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        h2 {
            class: "headline",
            ..attributes,
            {children}
        }
    }
}
