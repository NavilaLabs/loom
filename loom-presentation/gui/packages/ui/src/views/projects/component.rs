use crate::components::atoms::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::components::atoms::{Button, Input, Select, SelectOption, ToastMessage, Toasts};
use crate::layouts::DefaultLayout;
use api::customer::CustomerDto;
use api::project::ProjectDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{HiBriefcase, HiPencil, HiPlus, HiSave, HiX};
use dioxus_free_icons::Icon;

#[component]
pub fn Projects() -> Element {
    let mut projects = use_signal(Vec::<ProjectDto>::new);
    let mut customers = use_signal(Vec::<CustomerDto>::new);
    let mut toasts: Toasts = use_context();

    // Create form
    let mut new_name = use_signal(String::new);
    let mut new_customer_id = use_signal(|| Option::<String>::None);

    // Inline edit state
    let mut editing_id = use_signal(|| Option::<String>::None);
    let mut edit_name = use_signal(String::new);
    let mut edit_comment = use_signal(String::new);
    let mut edit_order_number = use_signal(String::new);
    let mut edit_visible = use_signal(|| true);
    let mut edit_billable = use_signal(|| true);
    let mut edit_time_budget = use_signal(String::new);
    let mut edit_money_budget = use_signal(String::new);
    let mut edit_budget_monthly = use_signal(|| false);

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
        let name = new_name.peek().clone();
        let cid = new_customer_id.peek().clone();
        if name.is_empty() || cid.is_none() {
            return;
        }
        match api::project::create_project(cid.unwrap(), name).await {
            Ok(dto) => {
                projects.write().push(dto);
                new_name.set(String::new());
                new_customer_id.set(None);
                toasts.write().push(ToastMessage::success("Project created"));
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
        let order_number = {
            let s = edit_order_number.peek().clone();
            if s.is_empty() { None } else { Some(s) }
        };
        let visible = *edit_visible.peek();
        let billable = *edit_billable.peek();

        if let Err(e) = api::project::update_project(
            id.clone(), name.clone(), comment.clone(), order_number.clone(), visible, billable,
        ).await {
            toasts.write().push(ToastMessage::error(e.to_string()));
            return;
        }

        // time_budget stored in seconds; GUI shows hours
        let time_budget: Option<i32> = edit_time_budget.peek().parse::<f64>().ok()
            .map(|h| (h * 3600.0) as i32);
        let money_budget: Option<i64> = edit_money_budget.peek().parse::<f64>().ok()
            .map(|v| (v * 100.0) as i64);
        let budget_monthly = *edit_budget_monthly.peek();
        if let Err(e) = api::project::set_project_budget(
            id.clone(), time_budget, money_budget, budget_monthly,
        ).await {
            toasts.write().push(ToastMessage::error(e.to_string()));
            return;
        }

        // Patch the item in-place — no extra round-trip needed
        let customer_id = projects.read()
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
        editing_id.set(None);
        toasts.write().push(ToastMessage::success("Project saved"));
    };

    rsx! {
        DefaultLayout {
            div { class: "space-y-6",

                // ── Create form ──────────────────────────────────────────────
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
                        div { class: "grid grid-cols-1 gap-4 md:grid-cols-2",
                            div { class: "form-field",
                                label { class: "form-label", "Customer" }
                                Select::<String> {
                                    options: customers.read().iter()
                                        .map(|c| SelectOption::new(c.id.clone(), c.name.clone()))
                                        .collect(),
                                    value: new_customer_id.read().clone(),
                                    on_change: move |id: String| new_customer_id.set(Some(id)),
                                    placeholder: "Select customer…".to_string(),
                                }
                            }
                            div { class: "form-field",
                                label { class: "form-label", r#for: "p-name", "Name" }
                                Input {
                                    id: "p-name",
                                    placeholder: "Website Redesign",
                                    value: new_name.read().clone(),
                                    oninput: move |e: FormEvent| new_name.set(e.value()),
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

                // ── Project list ─────────────────────────────────────────────
                div { class: "flex flex-col gap-3",
                    for project in projects.read().clone() {
                        {
                            let p = project.clone();
                            let pid = p.id.clone();
                            let is_editing = editing_id.read().as_deref() == Some(p.id.as_str());
                            let customer_name = customers.read()
                                .iter()
                                .find(|c| c.id == p.customer_id)
                                .map(|c| c.name.clone())
                                .unwrap_or_else(|| p.customer_id.clone());

                            if is_editing {
                                rsx! {
                                    Card { key: "{p.id}",
                                        CardHeader {
                                            CardTitle {
                                                div { class: "flex items-center justify-between",
                                                    span { "{p.name}" }
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
                                                div { class: "form-field md:col-span-2",
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
                                                div { class: "form-field flex items-center gap-3",
                                                    label { class: "form-label", "Monthly Budget" }
                                                    input {
                                                        r#type: "checkbox",
                                                        class: "form-checkbox",
                                                        checked: *edit_budget_monthly.read(),
                                                        oninput: move |_| { let v = *edit_budget_monthly.peek(); edit_budget_monthly.set(!v); },
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    Card { key: "{p.id}",
                                        CardContent {
                                            div { class: "flex items-center justify-between",
                                                div { class: "flex flex-col gap-1",
                                                    span { class: "font-medium", "{p.name}" }
                                                    span { class: "text-sm text-secondary", "{customer_name}" }
                                                    div { class: "flex gap-3 text-xs text-secondary",
                                                        if let Some(tb) = p.time_budget {
                                                            span { {format!("Time: {:.1}h", tb as f64 / 3600.0)} }
                                                        }
                                                        if let Some(mb) = p.money_budget {
                                                            span { "Budget: {mb / 100} EUR" }
                                                        }
                                                        if let Some(on) = &p.order_number {
                                                            span { "#{on}" }
                                                        }
                                                        if p.billable {
                                                            span { class: "text-success", "Billable" }
                                                        }
                                                        if !p.visible {
                                                            span { class: "text-warning", "Hidden" }
                                                        }
                                                    }
                                                }
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
                                                            editing_id.set(Some(pr.id));
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
