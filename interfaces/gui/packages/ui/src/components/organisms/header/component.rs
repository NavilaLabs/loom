use dioxus::prelude::*;

use crate::components::molecules::{LoomLogo, NavilaLabsLogo, Seperator};

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
            }
        }
    }
}
