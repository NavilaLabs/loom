use crate::components::atoms::{Button, Headline, Input, Label};
use crate::components::organisms::Header;
use crate::layouts::DefaultLayout;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    rsx! {
        DefaultLayout {
            Label { html_for: "username", "Username" }
            Input { id: "username" }
            Label { html_for: "password", "Password" }
            Input { id: "password", r#type: "password" }
            Button { r#type: "submit", "Submit" }
        }
    }
}
