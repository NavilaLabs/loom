use dioxus::prelude::*;

#[component]
pub fn Form(
    #[props(extends=GlobalAttributes)]
    #[props(extends=form)]
    attributes: Vec<Attribute>,
    children: Element,
    #[props(into, default)] class: String,
) -> Element {
    rsx! {
        form {
            class: format!("form w-full {}", class),
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn FormField(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    #[props(into, default)] class: String,
) -> Element {
    rsx! {
        div {
            class: format!("form-field w-full my-2 {}", class),
            ..attributes,
            {children}
        }
    }
}
