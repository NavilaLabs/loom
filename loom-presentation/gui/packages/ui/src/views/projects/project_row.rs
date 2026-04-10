use crate::components::atoms::{
    Button, Input, TableCell, TableExpandRow, TableRow, ToastExt, Toasts,
};
use crate::form_machine::{new_form, FormAction, State};
use api::customer::CustomerDto;
use api::project::ProjectDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{HiPencil, HiRefresh, HiSave, HiX};
use dioxus_free_icons::Icon;
use loom_core::{
    tenant::project::UpdateProjectInput,
    validation::{validation_summary, Validate},
};

#[derive(Clone, PartialEq, Props)]
pub(super) struct ProjectRowProps {
    pub project: ProjectDto,
    pub projects: Signal<Vec<ProjectDto>>,
    pub customers: Signal<Vec<CustomerDto>>,
    pub editing_id: Signal<Option<String>>,
    pub col_count: usize,
}

#[component]
pub(super) fn ProjectRow(props: ProjectRowProps) -> Element {
    let mut toasts: Toasts = use_context();

    let p = props.project.clone();
    let pid = p.id.clone();
    let mut projects = props.projects;
    let customers = props.customers;
    let mut editing_id = props.editing_id;
    let is_editing = editing_id.read().as_deref() == Some(p.id.as_str());

    let customer_name = customers
        .read()
        .iter()
        .find(|c| c.id == p.customer_id)
        .map(|c| c.name.clone())
        .unwrap_or_else(|| p.customer_id.clone());

    let mut edit_form = use_signal(new_form);
    let mut edit_name = use_signal(String::new);
    let mut edit_comment = use_signal(String::new);
    let mut edit_order_number = use_signal(String::new);
    let mut edit_visible = use_signal(|| true);
    let mut edit_billable = use_signal(|| true);
    let mut edit_time_budget = use_signal(String::new);
    let mut edit_money_budget = use_signal(String::new);
    let mut edit_budget_monthly = use_signal(|| false);

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
        let order_number = {
            let s = edit_order_number.peek().clone();
            if s.is_empty() { None } else { Some(s) }
        };
        let visible = *edit_visible.peek();
        let billable = *edit_billable.peek();

        edit_form.write().handle(&FormAction::Submit);
        if let Err(e) = (UpdateProjectInput { name: name.clone() }).validate() {
            edit_form
                .write()
                .handle(&FormAction::Fail(validation_summary(&e)));
            return;
        }

        if let Err(e) = api::project::update_project(
            id.clone(),
            name.clone(),
            comment.clone(),
            order_number.clone(),
            visible,
            billable,
        )
        .await
        {
            edit_form.write().handle(&FormAction::Fail(e.to_string()));
            toasts.push_error(e.to_string());
            return;
        }

        let time_budget: Option<i32> = edit_time_budget
            .peek()
            .parse::<f64>()
            .ok()
            .map(|h| (h * 3600.0) as i32);
        let money_budget: Option<i64> = edit_money_budget
            .peek()
            .parse::<f64>()
            .ok()
            .map(|v| (v * 100.0) as i64);
        let budget_monthly = *edit_budget_monthly.peek();
        if let Err(e) =
            api::project::set_project_budget(id.clone(), time_budget, money_budget, budget_monthly)
                .await
        {
            edit_form.write().handle(&FormAction::Fail(e.to_string()));
            toasts.push_error(e.to_string());
            return;
        }

        let customer_id = projects
            .read()
            .iter()
            .find(|x| x.id == id)
            .map(|p| p.customer_id.clone())
            .unwrap_or_default();
        let updated = api::project::ProjectDto {
            id: id.clone(),
            customer_id,
            name,
            comment,
            order_number,
            visible,
            billable,
            time_budget,
            money_budget,
            budget_is_monthly: budget_monthly,
        };
        if let Some(item) = projects.write().iter_mut().find(|x| x.id == id) {
            *item = updated;
        }
        edit_form
            .write()
            .handle(&FormAction::Succeed("Project saved".into()));
        editing_id.set(None);
        toasts.push_success("Project saved");
    };

    let edit_submitting = matches!(edit_form.read().state(), State::Submitting {});

    rsx! {
        TableRow { key: "{p.id}", muted: !p.visible,
            TableCell { span { class: "font-medium", "{p.name}" } }
            TableCell {
                span { class: "text-secondary text-sm", "{customer_name}" }
            }
            TableCell {
                div { class: "flex flex-col gap-0.5 text-xs text-secondary",
                    if let Some(tb) = p.time_budget {
                        span { {format!("{:.1}h time", tb as f64 / 3600.0)} }
                    }
                    if let Some(mb) = p.money_budget {
                        span { {format!("{:.0} EUR", mb as f64 / 100.0)} }
                    }
                    if let Some(ref on) = p.order_number {
                        span { "#{on}" }
                    }
                }
            }
            TableCell {
                div { class: "flex gap-2 text-xs",
                    if p.billable {
                        span { class: "text-success", "Billable" }
                    }
                    if !p.visible {
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
                            let proj = projects.read()
                                .iter()
                                .find(|x| x.id == pid)
                                .cloned();
                            if let Some(pr) = proj {
                                edit_name.set(pr.name.clone());
                                edit_comment.set(pr.comment.clone().unwrap_or_default());
                                edit_order_number.set(pr.order_number.clone().unwrap_or_default());
                                edit_visible.set(pr.visible);
                                edit_billable.set(pr.billable);
                                edit_time_budget.set(pr.time_budget.map(|v| format!("{:.1}", v as f64 / 3600.0)).unwrap_or_default());
                                edit_money_budget.set(pr.money_budget.map(|v| format!("{:.2}", v as f64 / 100.0)).unwrap_or_default());
                                edit_budget_monthly.set(pr.budget_is_monthly);
                                edit_form.write().handle(&FormAction::Reset);
                                editing_id.set(Some(pr.id));
                            }
                        },
                        Icon { icon: HiPencil, width: 14, height: 14 }
                    }
                }
            }
        }
        if is_editing {
            TableExpandRow { col_count: props.col_count,
                div { class: "grid grid-cols-1 gap-4 md:grid-cols-3",
                    div { class: "form-field",
                        label { class: "form-label", r#for: "ep-name", "Name" }
                        Input {
                            id: "ep-name",
                            value: edit_name.read().clone(),
                            oninput: move |e: FormEvent| edit_name.set(e.value()),
                        }
                    }
                    div { class: "form-field",
                        label { class: "form-label", r#for: "ep-order", "Order Number" }
                        Input {
                            id: "ep-order",
                            placeholder: "PO-2024-001",
                            value: edit_order_number.read().clone(),
                            oninput: move |e: FormEvent| edit_order_number.set(e.value()),
                        }
                    }
                    div { class: "form-field",
                        label { class: "form-label", r#for: "ep-comment", "Comment" }
                        Input {
                            id: "ep-comment",
                            placeholder: "Optional notes…",
                            value: edit_comment.read().clone(),
                            oninput: move |e: FormEvent| edit_comment.set(e.value()),
                        }
                    }
                    div { class: "form-field",
                        label { class: "form-label", r#for: "ep-time-budget", "Time Budget (hours)" }
                        Input {
                            id: "ep-time-budget",
                            placeholder: "e.g. 80",
                            value: edit_time_budget.read().clone(),
                            oninput: move |e: FormEvent| edit_time_budget.set(e.value()),
                        }
                    }
                    div { class: "form-field",
                        label { class: "form-label", r#for: "ep-money-budget", "Money Budget (EUR)" }
                        Input {
                            id: "ep-money-budget",
                            placeholder: "e.g. 10000.00",
                            value: edit_money_budget.read().clone(),
                            oninput: move |e: FormEvent| edit_money_budget.set(e.value()),
                        }
                    }
                    div { class: "form-field flex flex-col gap-2",
                        label { class: "form-label", "Options" }
                        div { class: "flex gap-4",
                            label { class: "flex items-center gap-2 text-sm",
                                input {
                                    r#type: "checkbox",
                                    class: "form-checkbox",
                                    checked: *edit_visible.read(),
                                    oninput: move |_| { let v = *edit_visible.peek(); edit_visible.set(!v); },
                                }
                                "Visible"
                            }
                            label { class: "flex items-center gap-2 text-sm",
                                input {
                                    r#type: "checkbox",
                                    class: "form-checkbox",
                                    checked: *edit_billable.read(),
                                    oninput: move |_| { let v = *edit_billable.peek(); edit_billable.set(!v); },
                                }
                                "Billable"
                            }
                            label { class: "flex items-center gap-2 text-sm",
                                input {
                                    r#type: "checkbox",
                                    class: "form-checkbox",
                                    checked: *edit_budget_monthly.read(),
                                    oninput: move |_| { let v = *edit_budget_monthly.peek(); edit_budget_monthly.set(!v); },
                                }
                                "Monthly Budget"
                            }
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
