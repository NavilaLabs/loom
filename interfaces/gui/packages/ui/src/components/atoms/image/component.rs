use dioxus::prelude::*;

#[component]
pub fn Image(
    src: String,
    alt: String,
    class: Option<String>,
    width: Option<i64>,
    height: Option<i64>,
    #[props(extends=GlobalAttributes)]
    #[props(extends=img)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        img {
            src,
            alt,
            class,
            width,
            height,
            loading: "lazy",
            ..attributes,
        }
    }
}
