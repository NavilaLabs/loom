use dioxus::{fullstack::routing::Route, prelude::*};

use crate::components::{
    atoms::{Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarTrigger},
    molecules::{LoomLogo, NavilaLabsLogo, Seperator},
};

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "header p-4 w-full",
            div { class: "header-content h-16 flex flex-row items-center justify-between",
                div { class: "logos flex flex-row items-center space-x-2",
                    NavilaLabsLogo { class: "h-16" }
                    Seperator { class: "h-16" }
                    LoomLogo { class: "h-16" }
                }
                Navbar {
                    NavbarItem {
                        index: 0usize,
                        value: "login".to_string(),
                        to: "/login",
                        "Login"
                    }
                    NavbarItem {
                        index: 0usize,
                        value: "database".to_string(),
                        to: "/developer/database",
                        "Database"
                    }
                }
            }
        }
    }
}
