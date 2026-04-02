use crate::components::atoms::{
    Button, ButtonVariant, Card, CardContent, CardFooter, Form, FormField, Headline, Input, Label,
    TabContent, TabList, TabTrigger, Tabs,
};
use crate::components::organisms::Header;
use crate::layouts::DefaultLayout;
use dioxus::prelude::*;

#[component]
pub fn Setup() -> Element {
    let mut active_tab = use_signal(|| Some("admin".to_string()));
    let mut username = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm_password = use_signal(String::new);
    let mut workspace_name = use_signal(String::new);

    rsx! {
        Header {}

        DefaultLayout {
            Card {
                class: "w-full",
                data_size: "md",
                Tabs {
                    value: active_tab,
                    default_value: "admin",
                    on_value_change: move |v: String| active_tab.set(Some(v)),
                    CardContent {
                        TabList {
                            class: "w-full mb-4",
                            TabTrigger { value: "admin", index: 0usize, "Admin User" },
                            TabTrigger { value: "workspace", index: 1usize, "Workspace" }
                        }
                        TabContent { value: "admin", index: 0usize,
                            Form {
                                FormField {
                                    Label { html_for: "username", class: "w-full", "Username" }
                                    Input { id: "username", class: "w-full", oninput: move |e: FormEvent| username.set(e.value()) }
                                }
                                FormField {
                                    Label { html_for: "email", class: "w-full", "Email" }
                                    Input { id: "email", r#type: "email", class: "w-full", oninput: move |e: FormEvent| email.set(e.value()) }
                                }
                                FormField {
                                    Label { html_for: "password", class: "w-full", "Password" }
                                    Input { id: "password", r#type: "password", class: "w-full", oninput: move |e: FormEvent| password.set(e.value()) }
                                }
                                FormField {
                                    Label { html_for: "confirm_password", class: "w-full", "Confirm Password" }
                                    Input { id: "confirm_password", r#type: "password", class: "w-full", oninput: move |e: FormEvent| confirm_password.set(e.value()) }
                                }
                            }
                        }
                        TabContent { value: "workspace", index: 1usize,
                            Form {
                                FormField {
                                    Label { html_for: "workspace_name", class: "w-full", "Workspace Name" }
                                    Input { id: "workspace_name", class: "w-full", oninput: move |e: FormEvent| workspace_name.set(e.value()) }
                                }
                            }
                        }
                    }
                }
                CardFooter {
                    class: "flex",
                    if active_tab.read().as_deref() == Some("workspace") {
                        Button {
                            variant: ButtonVariant::Secondary,
                            onclick: move |_| active_tab.set(Some("admin".to_string())),
                            "Back"
                        }
                    }
                    if active_tab.read().as_deref() == Some("admin") {
                        Button {
                            class: "ms-auto",
                            onclick: move |_| active_tab.set(Some("workspace".to_string())),
                            "Next"
                        }
                    } else {
                        Button { class: "ms-auto", r#type: "submit", "Submit" }
                    }
                }
            }
        }
    }
}
