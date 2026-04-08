use crate::components::atoms::{Button, Input};
use crate::components::atoms::card::{Card, CardContent, CardHeader, CardTitle};
use crate::layouts::DefaultLayout;
use api::activity::ActivityDto;
use dioxus::prelude::*;

#[component]
pub fn Activities() -> Element {
    let mut activities = use_signal(Vec::<ActivityDto>::new);
    let mut name = use_signal(String::new);
    let mut error = use_signal(|| Option::<String>::None);

    use_resource(move || async move {
        match api::activity::list_activities().await {
            Ok(list) => activities.set(list),
            Err(e) => error.set(Some(e.to_string())),
        }
    });

    let on_create = move |_| async move {
        let n = name.peek().clone();
        if n.is_empty() {
            return;
        }
        match api::activity::create_activity(None, n).await {
            Ok(dto) => {
                activities.write().push(dto);
                name.set(String::new());
            }
            Err(e) => error.set(Some(e.to_string())),
        }
    };

    rsx! {
        DefaultLayout {
            div { class: "space-y-6",
                // Create form
                div { class: "flex flex-col gap-2 p-4 border rounded-md",
                    h2 { class: "text-lg font-semibold", "New Activity" }
                    Input {
                        placeholder: "Activity name",
                        value: name.read().clone(),
                        oninput: move |e: FormEvent| name.set(e.value()),
                    }
                    Button { onclick: on_create, "Create" }
                    if let Some(err) = error.read().as_ref() {
                        p { class: "text-destructive text-sm", "{err}" }
                    }
                }

                // Activity list
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    for activity in activities.read().iter() {
                        {
                            let a = activity.clone();
                            rsx! {
                                Card { key: "{a.id}",
                                    CardHeader {
                                        CardTitle { "{a.name}" }
                                    }
                                    CardContent {
                                        if let Some(pid) = &a.project_id {
                                            p { class: "text-sm text-muted-foreground",
                                                "Project: {pid}"
                                            }
                                        }
                                        if !a.visible {
                                            p { class: "text-xs text-muted-foreground", "(hidden)" }
                                        }
                                        if a.billable {
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
