use crate::components::atoms::{
    Button, Input, TableCell, TableExpandRow, TableRow, ToastExt, Toasts,
};
use crate::form_machine::{new_form, FormAction, State};
use api::activity::ActivityDto;
use api::customer::CustomerDto;
use api::project::ProjectDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{HiPencil, HiRefresh, HiSave, HiX};
use dioxus_free_icons::Icon;
use loom_core::{
    tenant::activity::UpdateActivityInput,
    validation::{validation_summary, Validate},
};

#[derive(Clone, PartialEq, Props)]
pub(super) struct ActivityRowProps {
    pub activity: ActivityDto,
    pub activities: Signal<Vec<ActivityDto>>,
    pub projects: Signal<Vec<ProjectDto>>,
    pub customers: Signal<Vec<CustomerDto>>,
    pub editing_id: Signal<Option<String>>,
    pub col_count: usize,
}

#[component]
pub(super) fn ActivityRow(props: ActivityRowProps) -> Element {
    let mut toasts: Toasts = use_context();

    let a = props.activity.clone();
    let aid = a.id.clone();
    let mut activities = props.activities;
    let projects = props.projects;
    let mut editing_id = props.editing_id;
    let is_editing = editing_id.read().as_deref() == Some(a.id.as_str());

    let project_name = a.project_id.as_ref().and_then(|pid| {
        projects.read().iter().find(|p| &p.id == pid).map(|p| p.name.clone())
    });

    let mut edit_form = use_signal(new_form);
    let mut edit_name = use_signal(String::new);
    let mut edit_comment = use_signal(String::new);
    let mut edit_visible = use_signal(|| true);
    let mut edit_billable = use_signal(|| true);

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

        edit_form.write().handle(&FormAction::Submit);
        if let Err(e) = (UpdateActivityInput { name: name.clone() }).validate() {
            edit_form
                .write()
                .handle(&FormAction::Fail(validation_summary(&e)));
            return;
        }

        if let Err(e) =
            api::activity::update_activity(id.clone(), name, comment, visible, billable).await
        {
            edit_form.write().handle(&FormAction::Fail(e.to_string()));
            toasts.push_error(e.to_string());
            return;
        }

        match api::activity::list_activities().await {
            Ok(list) => activities.set(list),
            Err(e) => toasts.push_error(e.to_string()),
        }
        edit_form
            .write()
            .handle(&FormAction::Succeed("Activity saved".into()));
        editing_id.set(None);
        toasts.push_success("Activity saved");
    };

    let edit_submitting = matches!(edit_form.read().state(), State::Submitting {});

    rsx! {
        TableRow { key: "{a.id}", muted: !a.visible,
            TableCell { "{a.name}" }
            TableCell {
                if let Some(pn) = project_name {
                    span { "{pn}" }
                } else {
                    span { class: "text-secondary text-xs", "Global" }
                }
            }
            TableCell {
                div { class: "flex gap-2 text-xs",
                    if a.billable {
                        span { class: "text-success", "Billable" }
                    }
                    if !a.visible {
                        span { class: "text-warning", "Hidden" }
                    }
                }
            }
            TableCell {
                if is_editing {
                    Button {
                        onclick: move |_| editing_id.set(None),
                        Icon { icon: HiX, width: 14, height: 14 }
                    }
                } else {
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
                                edit_form.write().handle(&FormAction::Reset);
                                editing_id.set(Some(ac.id));
                            }
                        },
                        Icon { icon: HiPencil, width: 14, height: 14 }
                    }
                }
            }
        }
        if is_editing {
            TableExpandRow { col_count: props.col_count,
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
                if matches!(edit_form.read().state(), State::Error {}) {
                    p { class: "text-red-500 text-sm mt-2",
                        "{edit_form.read().message}"
                    }
                }
                div { class: "flex gap-2 mt-2",
                    Button {
                        onclick: on_save,
                        disabled: edit_submitting,
                        if edit_submitting {
                            Icon { icon: HiRefresh, width: 14, height: 14 }
                            "Saving…"
                        } else {
                            Icon { icon: HiSave, width: 14, height: 14 }
                            "Save"
                        }
                    }
                    Button {
                        onclick: move |_| editing_id.set(None),
                        Icon { icon: HiX, width: 14, height: 14 }
                        "Cancel"
                    }
                }
            }
        }
    }
}
