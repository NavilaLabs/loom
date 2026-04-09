use crate::components::atoms::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::components::atoms::{Button, Input, ToastMessage, Toasts};
use crate::layouts::DefaultLayout;
use api::activity::ActivityDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{HiPlus, HiTag};
use dioxus_free_icons::Icon;

#[component]
pub fn Activities() -> Element {
    let mut activities = use_signal(Vec::<ActivityDto>::new);
    let mut name = use_signal(String::new);
    let mut toasts: Toasts = use_context();

    use_resource(move || async move {
        match api::activity::list_activities().await {
            Ok(list) => activities.set(list),
            Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
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
                toasts.write().push(ToastMessage::success("Activity created"));
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
                                Icon { icon: HiTag, width: 18, height: 18 }
                                "New Activity"
                            }
                        }
                    }
                    CardContent {
                        div { class: "form-field",
                            label { class: "form-label", r#for: "activity-name", "Name" }
                            Input {
                                id: "activity-name",
                                placeholder: "Development",
                                value: name.read().clone(),
                                oninput: move |e: FormEvent| name.set(e.value()),
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
