use crate::components::atoms::Button;
use crate::components::atoms::card::{Card, CardContent};
use crate::layouts::DefaultLayout;
use api::workspace::WorkspaceDto;
use dioxus::prelude::*;

type AuthState = Signal<Option<Option<api::auth::UserInfo>>>;

#[component]
pub fn SelectWorkspace() -> Element {
    let mut workspaces = use_signal(Vec::<WorkspaceDto>::new);
    let mut error = use_signal(|| None::<String>);
    let mut auth: AuthState = use_context();
    let navigator = use_navigator();

    use_resource(move || async move {
        match api::workspace::list_workspaces().await {
            Ok(list) => workspaces.set(list),
            Err(e) => error.set(Some(e.to_string())),
        }
    });

    rsx! {
        DefaultLayout {
            div { class: "space-y-4 max-w-md mx-auto",
                p { class: "text-muted-foreground text-sm", "Select a workspace to continue." }

                for ws in workspaces.read().iter() {
                    {
                        let ws = ws.clone();
                        let id = ws.id.clone();
                        rsx! {
                            Card { key: "{ws.id}",
                                CardContent {
                                    div { class: "flex items-center justify-between py-1",
                                        p { class: "font-medium",
                                            { ws.name.as_deref().unwrap_or("Unnamed workspace") }
                                        }
                                        Button {
                                            onclick: move |_| {
                                                let id = id.clone();
                                                async move {
                                                    match api::workspace::select_workspace(id).await {
                                                        Ok(()) => {
                                                            // Refresh auth so workspace_id propagates app-wide.
                                                            if let Ok(user) = api::auth::get_current_user().await {
                                                                auth.set(Some(user));
                                                            }
                                                            navigator.push("/dashboard");
                                                        }
                                                        Err(e) => error.set(Some(e.to_string())),
                                                    }
                                                }
                                            },
                                            "Select"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if let Some(msg) = error.read().as_ref() {
                    p { class: "text-red-500 text-sm", "{msg}" }
                }
            }
        }
    }
}
