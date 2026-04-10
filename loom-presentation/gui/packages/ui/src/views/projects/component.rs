use crate::components::atoms::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::components::atoms::{
    Button, ColumnDef, DataTable, Input, Select, SelectOption, TableCell, TableExpandRow, TableRow,
    ToastExt, Toasts,
};
use crate::form_machine::{new_form, FormAction, State};
use crate::layouts::DefaultLayout;
use crate::{CustomersCache, ProjectsCache};
use api::project::ProjectDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{
    HiBriefcase, HiPencil, HiPlus, HiRefresh, HiSave, HiX,
};
use dioxus_free_icons::Icon;
use loom_core::{
    tenant::project::{CreateProjectInput, UpdateProjectInput},
    validation::{validation_summary, Validate},
};

const PAGE_SIZE: usize = 15;

#[component]
pub fn Projects() -> Element {
    let projects_cache: ProjectsCache = use_context();
    let customers_cache: CustomersCache = use_context();
    let mut projects = use_signal(|| projects_cache.read().clone());
    let mut customers = use_signal(|| customers_cache.read().clone());
    let mut loading = use_signal(|| projects_cache.read().is_empty());
    let mut toasts: Toasts = use_context();
    let mut page = use_signal(|| 0_usize);

    // State machine drives the create-form lifecycle.
    let mut create_form = use_signal(new_form);

    // Create form
    let mut new_name = use_signal(String::new);
    let mut new_customer_id = use_signal(|| Option::<String>::None);

    // Inline edit state
    let mut editing_id = use_signal(|| Option::<String>::None);
    let mut edit_form = use_signal(new_form);
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
            Err(e) => toasts.push_error(e.to_string()),
        }
        match api::customer::list_customers().await {
            Ok(list) => customers.set(list),
            Err(e) => toasts.push_error(e.to_string()),
        }
        loading.set(false);
    });


    let on_create = move |_| async move {
        let name = new_name.peek().clone();
        let cid = new_customer_id.peek().clone();

        create_form.write().handle(&FormAction::Submit);
        if cid.is_none() {
            create_form
                .write()
                .handle(&FormAction::Fail("Please select a customer".into()));
            return;
        }
        if let Err(e) = (CreateProjectInput { name: name.clone() }).validate() {
            create_form
                .write()
                .handle(&FormAction::Fail(validation_summary(&e)));
            return;
        }
        match api::project::create_project(cid.unwrap(), name).await {
            Ok(dto) => {
                projects.write().push(dto);
                new_name.set(String::new());
                new_customer_id.set(None);
                create_form
                    .write()
                    .handle(&FormAction::Succeed("Project created".into()));
                toasts.push_success("Project created");
            }
            Err(e) => {
                create_form.write().handle(&FormAction::Fail(e.to_string()));
                toasts.push_error(e.to_string());
            }
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

    let create_submitting = matches!(create_form.read().state(), State::Submitting {});

    // Pagination slice
    let total = projects.read().len();
    let current_page = *page.read();
    let page_items: Vec<ProjectDto> = projects
        .read()
        .iter()
        .skip(current_page * PAGE_SIZE)
        .take(PAGE_SIZE)
        .cloned()
        .collect();

    let columns = vec![
        ColumnDef::new("Name"),
        ColumnDef::new("Customer"),
        ColumnDef::new("Budget"),
        ColumnDef::new("Flags"),
        ColumnDef::new("").width("80px"),
    ];
    let col_count = columns.len();

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
                        if matches!(create_form.read().state(), State::Error {}) {
                            p { class: "text-red-500 text-sm mt-2",
                                "{create_form.read().message}"
                            }
                        }
                    }
                    CardFooter {
                        Button {
                            onclick: on_create,
                            disabled: create_submitting,
                            if create_submitting {
                                Icon { icon: HiRefresh, width: 16, height: 16 }
                                "Creating…"
                            } else {
                                Icon { icon: HiPlus, width: 16, height: 16 }
                                "Create"
                            }
                        }
                    }
                }

                // ── Project list ─────────────────────────────────────────────
                div { class: "island",
                    div { class: "island-header",
                        span { class: "island-title", "Projects" }
                    }
                    DataTable {
                        columns,
                        total,
                        page: current_page,
                        page_size: PAGE_SIZE,
                        loading: *loading.read(),
                        on_page_change: move |p| page.set(p),

                        for project in page_items {
                            {
                                let p = project.clone();
                                let pid = p.id.clone();
                                let is_editing = editing_id.read().as_deref() == Some(p.id.as_str());
                                let customer_name = customers.read()
                                    .iter()
                                    .find(|c| c.id == p.customer_id)
                                    .map(|c| c.name.clone())
                                    .unwrap_or_else(|| p.customer_id.clone());
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
                                        TableExpandRow { col_count,
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
                        }
                    }
                }
            }
        }
    }
}
