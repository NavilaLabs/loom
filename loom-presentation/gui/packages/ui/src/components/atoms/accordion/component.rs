use dioxus::prelude::*;
use dioxus_primitives::accordion::{
    self, AccordionContentProps, AccordionItemProps, AccordionProps, AccordionTriggerProps,
};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let base = attributes!(div { class: "accordion" });
    let merged = merge_attributes(vec![base, props.attributes.clone()]);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        accordion::Accordion {
            id: props.id,
            allow_multiple_open: props.allow_multiple_open,
            disabled: props.disabled,
            collapsible: props.collapsible,
            horizontal: props.horizontal,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    let base = attributes!(div { class: "accordion-item" });
    let merged = merge_attributes(vec![base, props.attributes.clone()]);

    rsx! {
        accordion::AccordionItem {
            disabled: props.disabled,
            default_open: props.default_open,
            on_change: props.on_change,
            on_trigger_click: props.on_trigger_click,
            index: props.index,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    let base = attributes!(button { class: "accordion-trigger" });
    let merged = merge_attributes(vec![base, props.attributes.clone()]);

    rsx! {
        accordion::AccordionTrigger {
            id: props.id,
            attributes: merged,
            {props.children}
            svg {
                class: "accordion-expand-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    let base = attributes!(div {
        class: "accordion-content",
        style: "--collapsible-content-width: 50vh"
    });
    let merged = merge_attributes(vec![base, props.attributes.clone()]);

    rsx! {
        accordion::AccordionContent {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}
