use crate::components::atoms::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::components::atoms::{Button, Input, Select, SelectOption, ToastMessage, Toasts};
use crate::layouts::DefaultLayout;
use api::customer::CustomerDto;
use api::project::ProjectDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{HiBriefcase, HiPlus};
use dioxus_free_icons::Icon;

#[component]
pub fn Projects() -> Element {
    let mut projects = use_signal(Vec::<ProjectDto>::new);
    let mut customers = use_signal(Vec::<CustomerDto>::new);
    let mut name = use_signal(String::new);
    let mut customer_id = use_signal(|| Option::<String>::None);
    let mut toasts: Toasts = use_context();

    use_resource(move || async move {
        match api::project::list_projects().await {
            Ok(list) => projects.set(list),
            Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
        }
        match api::customer::list_customers().await {
            Ok(list) => customers.set(list),
            Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
        }
    });

    let on_create = move |_| async move {
        let n = name.peek().clone();
        let cid = customer_id.peek().clone();
        if n.is_empty() || cid.is_none() {
            return;
        }
        match api::project::create_project(cid.unwrap(), n).await {
            Ok(dto) => {
                projects.write().push(dto);
                name.set(String::new());
                customer_id.set(None);
                toasts.write().push(ToastMessage::success("Project created"));
            }
            Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
        }
    };

    rsx! {
        DefaultLayout {
            div { class: "space-y-6",
                // Create form
                Card { data_size: "md",
                    CardHeader {
                        CardTitle {
                            div { class: "flex items-center gap-2",
                                Icon { icon: HiBriefcase, width: 18, height: 18 }
                                "New Project"
                            }
                        }
                    }
                    CardContent {
                        div { class: "flex flex-col gap-4",
                            div { class: "form-field",
                                label { class: "form-label", "Customer" }
                                Select::<String> {
                                    options: customers.read().iter()
                                        .map(|c| SelectOption::new(c.id.clone(), c.name.clone()))
                                        .collect(),
                                    value: customer_id.read().clone(),
                                    on_change: move |id: String| customer_id.set(Some(id)),
                                    placeholder: "Select customer…".to_string(),
                                }
                            }
                            div { class: "form-field",
                                label { class: "form-label", r#for: "project-name", "Name" }
                                Input {
                                    id: "project-name",
                                    placeholder: "Website Redesign",
                                    value: name.read().clone(),
                                    oninput: move |e: FormEvent| name.set(e.value()),
                                }
                            }
                        }
                    }
                    CardFooter {
                        Button { onclick: on_create,
                            Icon { icon: HiPlus, width: 16, height: 16 }
                            "Create"
                        }
                    }
                }

                // Project list
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    for project in projects.read().iter() {
                        {
                            let p = project.clone();
                            // Look up customer name for display
                            let customer_name = customers.read()
                                .iter()
                                .find(|c| c.id == p.customer_id)
                                .map(|c| c.name.clone())
                                .unwrap_or_else(|| p.customer_id.clone());
                            rsx! {
                                Card { key: "{p.id}",
                                    CardHeader {
                                        CardTitle { "{p.name}" }
                                    }
                                    CardContent {
                                        p { class: "text-sm text-muted-foreground",
                                            "Customer: {customer_name}"
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
