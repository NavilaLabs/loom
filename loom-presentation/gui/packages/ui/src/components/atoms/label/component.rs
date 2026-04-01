use dioxus::{core::AttributeValue, prelude::*};
use dioxus_i18n::fluent::types::AnyEq;
use dioxus_primitives::label::{self, LabelProps};

#[component]
pub fn Label(
    LabelProps {
        html_for,
        attributes,
        children,
    }: LabelProps,
) -> Element {
    let extra_class = attributes
        .iter()
        .find(|a| a.name == "class")
        .and_then(|a| match &a.value {
            AttributeValue::Text(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or("");

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        label::Label {
            class: format!("label {}", extra_class),
            html_for: html_for,
            attributes: attributes,
            {children}
        }
    }
}
