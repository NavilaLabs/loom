use dioxus::prelude::*;

#[component]
pub fn Card(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    #[props(into, default)] class: String,
    #[props(into, default)] data_size: Option<String>,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: format!("card {}", class),
            "data-slot": "card",
            "data-size": data_size,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardHeader(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    #[props(into, default)] class: String,
) -> Element {
    rsx! {
        div {
            class: format!("card-header {}", class),
            "data-slot": "card-header",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardTitle(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    #[props(into, default)] class: String,
) -> Element {
    rsx! {
        div {
            class: format!("card-title {}", class),
            "data-slot": "card-title",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardDescription(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    #[props(into, default)] class: String,
) -> Element {
    rsx! {
        div {
            class: format!("card-description {}", class),
            "data-slot": "card-description",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardAction(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    #[props(into, default)] class: String,
) -> Element {
    rsx! {
        div {
            class: format!("card-action {}", class),
            "data-slot": "card-action",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardContent(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    #[props(into, default)] class: String,
) -> Element {
    rsx! {
        div {
            class: format!("card-content {}", class),
            "data-slot": "card-content",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardFooter(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    #[props(into, default)] class: String,
) -> Element {
    rsx! {
        div {
            class: format!("card-footer {}", class),
            "data-slot": "card-footer",
            ..attributes,
            {children}
        }
    }
}
