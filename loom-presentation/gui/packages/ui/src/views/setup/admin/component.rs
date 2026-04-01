use crate::components::atoms::{
    Button, Card, CardContent, CardFooter, Form, FormField, Headline, Input, Label,
};
use crate::layouts::DefaultLayout;
use dioxus::prelude::*;

#[component]
pub fn Admin() -> Element {
    rsx! {
        DefaultLayout {
            headline: rsx! { Headline { "Setup Admin" } },

            Card {
                class: "w-full",
                data_size: "md",
                CardContent {
                    Form {
                        FormField {
                            Label { html_for: "username", class: "w-full", "Username" }
                            Input { id: "username", class: "w-full" }
                        }
                        FormField {
                            Label { html_for: "email", class: "w-full", "Email" }
                            Input { id: "email", r#type: "email", class: "w-full" }
                        }
                        FormField {
                            Label { html_for: "password", class: "w-full", "Password" }
                            Input { id: "password", r#type: "password", class: "w-full" }
                        }
                    }
                }
                CardFooter {
                    class: "self-end",
                    Button { r#type: "submit", "Submit" }
                }
            }
        }
    }
}
