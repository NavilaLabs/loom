use dioxus::prelude::*;

use crate::components::{
    atoms::{Navbar, NavbarItem},
    molecules::{LoomLogo, NavilaLabsLogo, Seperator},
};

/// Mirrors the `AuthState` type alias from the `web` crate.
/// Both sides must use `Signal<Option<Option<api::auth::UserInfo>>>`.
type AuthState = Signal<Option<Option<api::auth::UserInfo>>>;

#[component]
pub fn Header() -> Element {
    let auth: AuthState = use_context();
    let user = auth.cloned().flatten();

    let nav = use_navigator();

    let mut auth: AuthState = use_context();

    let on_logout = move |_| async move {
        let _ = api::auth::logout().await;
        auth.set(Some(None));
        nav.replace("/login");
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        header { class: "header p-4 w-full",
            div { class: "header-content h-16 flex flex-row items-center justify-between",
                div { class: "logos flex flex-row items-center space-x-2",
                    NavilaLabsLogo { class: "h-16" }
                    Seperator { class: "h-16" }
                    LoomLogo { class: "h-16" }
                }

                Navbar {
                    match user {
                        None => rsx! {
                            NavbarItem {
                                index: 0usize,
                                value: "login".to_string(),
                                to: "/login",
                                "Login"
                            }
                        },
                        Some(ref u) => rsx! {
                            NavbarItem {
                                index: 0usize,
                                value: "dashboard".to_string(),
                                to: "/dashboard",
                                "Dashboard"
                            }
                            if u.is_admin {
                                NavbarItem {
                                    index: 1usize,
                                    value: "database".to_string(),
                                    to: "/developer/database",
                                    "Database"
                                }
                            }
                            button {
                                class: "navbar-item",
                                onclick: on_logout,
                                "Logout"
                            }
                        },
                    }
                }
            }
        }
    }
}
