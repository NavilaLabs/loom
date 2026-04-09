use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{
    HiBriefcase, HiClock, HiHome, HiLogout, HiOfficeBuilding, HiTag,
};
use dioxus_free_icons::Icon;

use crate::components::atoms::{Button, ButtonVariant, Navbar, NavbarItem};

/// Mirrors the `AuthState` type alias from the `web` crate.
type AuthState = Signal<Option<Option<api::auth::UserInfo>>>;

#[component]
pub fn Sidebar() -> Element {
    let auth: AuthState = use_context();
    let user = auth.cloned().flatten();

    let nav = use_navigator();
    let mut auth: AuthState = use_context();

    let on_logout = move |_| async move {
        let _ = api::auth::logout().await;
        auth.set(Some(None));
        nav.replace("/login");
    };

    // Only render the sidebar when the user has an active workspace session.
    let Some(user) = user else { return rsx! {} };
    if user.workspace_id.is_none() {
        return rsx! {};
    }

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        aside { class: "sidebar",
            div { class: "sidebar-top",
                div { class: "sidebar-brand",
                    span { class: "sidebar-brand-name", "Loom" }
                    span { class: "sidebar-brand-sub", "Curated Workspace" }
                }
                Navbar {
                    class: "sidebar-nav",
                    NavbarItem {
                        index: 0usize,
                        value: "dashboard".to_string(),
                        to: "/dashboard",
                        Icon { icon: HiHome, width: 16, height: 16 }
                        "Dashboard"
                    }
                    NavbarItem {
                        index: 1usize,
                        value: "timesheets".to_string(),
                        to: "/timesheets",
                        Icon { icon: HiClock, width: 16, height: 16 }
                        "Timesheets"
                    }
                    NavbarItem {
                        index: 2usize,
                        value: "customers".to_string(),
                        to: "/customers",
                        Icon { icon: HiOfficeBuilding, width: 16, height: 16 }
                        "Customers"
                    }
                    NavbarItem {
                        index: 3usize,
                        value: "projects".to_string(),
                        to: "/projects",
                        Icon { icon: HiBriefcase, width: 16, height: 16 }
                        "Projects"
                    }
                    NavbarItem {
                        index: 4usize,
                        value: "activities".to_string(),
                        to: "/activities",
                        Icon { icon: HiTag, width: 16, height: 16 }
                        "Activities"
                    }
                }
            }
            div { class: "sidebar-footer",
                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: on_logout,
                    Icon { icon: HiLogout, width: 16, height: 16 }
                    "Logout"
                }
            }
        }
    }
}
