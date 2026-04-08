use crate::components::atoms::{Button, Input};
use crate::components::atoms::card::{Card, CardContent, CardHeader, CardTitle};
use crate::layouts::DefaultLayout;
use api::project::ProjectDto;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    let mut projects = use_signal(Vec::<ProjectDto>::new);
    let mut name = use_signal(String::new);
    let mut customer_id = use_signal(String::new);
    let mut error = use_signal(|| Option::<String>::None);

    use_resource(move || async move {
        match api::project::list_projects().await {
            Ok(list) => projects.set(list),
            Err(e) => error.set(Some(e.to_string())),
        }
    });

    let on_create = move |_| async move {
        let n = name.peek().clone();
        let cid = customer_id.peek().clone();
        if n.is_empty() || cid.is_empty() {
            return;
        }
        match api::project::create_project(cid, n).await {
            Ok(dto) => {
                projects.write().push(dto);
                name.set(String::new());
                customer_id.set(String::new());
            }
            Err(e) => error.set(Some(e.to_string())),
        }
    };

    rsx! {
        DefaultLayout {
            div { class: "space-y-6",
                // Create form
                div { class: "flex flex-col gap-2 p-4 border rounded-md",
                    h2 { class: "text-lg font-semibold", "New Project" }
                    Input {
                        placeholder: "Customer ID",
                        value: customer_id.read().clone(),
                        oninput: move |e: FormEvent| customer_id.set(e.value()),
                    }
                    Input {
                        placeholder: "Project name",
                        value: name.read().clone(),
                        oninput: move |e: FormEvent| name.set(e.value()),
                    }
                    Button { onclick: on_create, "Create" }
                    if let Some(err) = error.read().as_ref() {
                        p { class: "text-destructive text-sm", "{err}" }
                    }
                }

                // Project list
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    for project in projects.read().iter() {
                        {
                            let p = project.clone();
                            rsx! {
                                Card { key: "{p.id}",
                                    CardHeader {
                                        CardTitle { "{p.name}" }
                                    }
                                    CardContent {
                                        p { class: "text-sm text-muted-foreground",
                                            "Customer: {p.customer_id}"
                                        }
                                        if !p.visible {
                                            p { class: "text-xs text-muted-foreground", "(hidden)" }
                                        }
                                        if p.billable {
                                            p { class: "text-xs text-green-600", "Billable" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
