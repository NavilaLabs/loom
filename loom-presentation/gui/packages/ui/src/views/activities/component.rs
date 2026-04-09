use crate::components::atoms::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::components::atoms::{Button, Input, Select, SelectOption, ToastMessage, Toasts};
use crate::layouts::DefaultLayout;
use api::activity::ActivityDto;
use api::project::ProjectDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{HiPencil, HiPlus, HiSave, HiTag, HiX};
use dioxus_free_icons::Icon;

#[component]
pub fn Activities() -> Element {
    let mut activities = use_signal(Vec::<ActivityDto>::new);
    let mut projects = use_signal(Vec::<ProjectDto>::new);
    let mut toasts: Toasts = use_context();

    // Create form
    let mut new_name = use_signal(String::new);
    let mut new_comment = use_signal(String::new);
    let mut new_project_id = use_signal(|| Option::<String>::None);
    let mut new_billable = use_signal(|| true);

    // Inline edit state
    let mut editing_id = use_signal(|| Option::<String>::None);
    let mut edit_name = use_signal(String::new);
    let mut edit_comment = use_signal(String::new);
    let mut edit_visible = use_signal(|| true);
    let mut edit_billable = use_signal(|| true);

    use_resource(move || async move {
        match api::activity::list_activities().await {
            Ok(list) => activities.set(list),
            Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
        }
        match api::project::list_projects().await {
            Ok(list) => projects.set(list),
            Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
        }
    });

    let on_create = move |_| async move {
        let name = new_name.peek().clone();
        if name.is_empty() {
            return;
        }
        let project_id = new_project_id.peek().clone();
        match api::activity::create_activity(project_id, name).await {
            Ok(dto) => {
                activities.write().push(dto);
                new_name.set(String::new());
                new_comment.set(String::new());
                new_project_id.set(None);
                toasts.write().push(ToastMessage::success("Activity created"));
            }
            Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
        }
    };

    let on_save = move |_| async move {
        let id = match editing_id.peek().clone() {
            Some(id) => id,
            None => return,
        };
        let name = edit_name.peek().clone();
        let comment = {
            let s = edit_comment.peek().clone();
            if s.is_empty() { None } else { Some(s) }
        };
        let visible = *edit_visible.peek();
        let billable = *edit_billable.peek();

        if let Err(e) = api::activity::update_activity(id.clone(), name, comment, visible, billable).await {
            toasts.write().push(ToastMessage::error(e.to_string()));
            return;
        }

        match api::activity::list_activities().await {
            Ok(list) => activities.set(list),
            Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
        }
        editing_id.set(None);
        toasts.write().push(ToastMessage::success("Activity saved"));
    };

    rsx! {
        DefaultLayout {
            div { class: "space-y-6",

                // ── Create form ──────────────────────────────────────────────
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
                        div { class: "grid grid-cols-1 gap-4 md:grid-cols-2",
                            div { class: "form-field",
                                label { class: "form-label", r#for: "a-name", "Name" }
                                Input {
                                    id: "a-name",
                                    placeholder: "Development",
                                    value: new_name.read().clone(),
                                    oninput: move |e: FormEvent| new_name.set(e.value()),
                                }
                            }
                            div { class: "form-field",
                                label { class: "form-label", "Project (optional)" }
                                Select::<String> {
                                    options: {
                                        let mut opts = vec![SelectOption::new("".to_string(), "— Global —".to_string())];
                                        opts.extend(projects.read().iter().map(|p| SelectOption::new(p.id.clone(), p.name.clone())));
                                        opts
                                    },
                                    value: new_project_id.read().clone(),
                                    on_change: move |id: String| {
                                        new_project_id.set(if id.is_empty() { None } else { Some(id) })
                                    },
                                    placeholder: "Global activity…".to_string(),
                                }
                            }
                            div { class: "form-field md:col-span-2",
                                label { class: "form-label", r#for: "a-comment", "Comment" }
                                Input {
                                    id: "a-comment",
                                    placeholder: "Optional description…",
                                    value: new_comment.read().clone(),
                                    oninput: move |e: FormEvent| new_comment.set(e.value()),
                                }
                            }
                            div { class: "form-field flex items-center gap-3",
                                label { class: "form-label", "Billable by default" }
                                input {
                                    r#type: "checkbox",
                                    class: "form-checkbox",
                                    checked: *new_billable.read(),
                                    oninput: move |_| { let v = *new_billable.peek(); new_billable.set(!v); },
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

                // ── Activity list ────────────────────────────────────────────
                div { class: "flex flex-col gap-3",
                    for activity in activities.read().clone() {
                        {
                            let a = activity.clone();
                            let aid = a.id.clone();
                            let is_editing = editing_id.read().as_deref() == Some(a.id.as_str());
                            let project_name = a.project_id.as_ref().and_then(|pid| {
                                projects.read().iter().find(|p| &p.id == pid).map(|p| p.name.clone())
                            });

                            if is_editing {
                                rsx! {
                                    Card { key: "{a.id}",
                                        CardHeader {
                                            CardTitle {
                                                div { class: "flex items-center justify-between",
                                                    span { "{a.name}" }
                                                    div { class: "flex gap-2",
                                                        Button { onclick: on_save,
                                                            Icon { icon: HiSave, width: 15, height: 15 }
                                                            "Save"
                                                        }
                                                        Button {
                                                            onclick: move |_| editing_id.set(None),
                                                            Icon { icon: HiX, width: 15, height: 15 }
                                                            "Cancel"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        CardContent {
                                            div { class: "grid grid-cols-1 gap-4 md:grid-cols-2",
                                                div { class: "form-field",
                                                    label { class: "form-label", r#for: "ea-name", "Name" }
                                                    Input {
                                                        id: "ea-name",
                                                        value: edit_name.read().clone(),
                                                        oninput: move |e: FormEvent| edit_name.set(e.value()),
                                                    }
                                                }
                                                div { class: "form-field",
                                                    label { class: "form-label", r#for: "ea-comment", "Comment" }
                                                    Input {
                                                        id: "ea-comment",
                                                        placeholder: "Optional description…",
                                                        value: edit_comment.read().clone(),
                                                        oninput: move |e: FormEvent| edit_comment.set(e.value()),
                                                    }
                                                }
                                                div { class: "form-field flex items-center gap-3",
                                                    label { class: "form-label", "Visible" }
                                                    input {
                                                        r#type: "checkbox",
                                                        class: "form-checkbox",
                                                        checked: *edit_visible.read(),
                                                        oninput: move |_| { let v = *edit_visible.peek(); edit_visible.set(!v); },
                                                    }
                                                }
                                                div { class: "form-field flex items-center gap-3",
                                                    label { class: "form-label", "Billable" }
                                                    input {
                                                        r#type: "checkbox",
                                                        class: "form-checkbox",
                                                        checked: *edit_billable.read(),
                                                        oninput: move |_| { let v = *edit_billable.peek(); edit_billable.set(!v); },
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    Card { key: "{a.id}",
                                        CardContent {
                                            div { class: "flex items-center justify-between",
                                                div { class: "flex flex-col gap-1",
                                                    span { class: "font-medium", "{a.name}" }
                                                    if let Some(pname) = project_name {
                                                        span { class: "text-sm text-secondary", "{pname}" }
                                                    }
                                                    div { class: "flex gap-3 text-xs text-secondary",
                                                        if let Some(c) = &a.comment {
                                                            span { "{c}" }
                                                        }
                                                        if a.billable {
                                                            span { class: "text-success", "Billable" }
                                                        }
                                                        if !a.visible {
                                                            span { class: "text-warning", "Hidden" }
                                                        }
                                                    }
                                                }
                                                Button {
                                                    onclick: move |_| {
                                                        let act = activities.read()
                                                            .iter()
                                                            .find(|x| x.id == aid)
                                                            .cloned();
                                                        if let Some(ac) = act {
                                                            edit_name.set(ac.name.clone());
                                                            edit_comment.set(ac.comment.clone().unwrap_or_default());
                                                            edit_visible.set(ac.visible);
                                                            edit_billable.set(ac.billable);
                                                            editing_id.set(Some(ac.id));
                                                        }
                                                    },
                                                    Icon { icon: HiPencil, width: 15, height: 15 }
                                                    "Edit"
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
    }
}
