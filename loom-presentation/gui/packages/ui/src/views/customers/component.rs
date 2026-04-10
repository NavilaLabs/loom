use crate::components::atoms::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::components::atoms::{
    Button, ColumnDef, DataTable, Input, SearchableSelect, Select, TableCell, TableExpandRow,
    TableRow, ToastExt, Toasts,
};
use crate::form_machine::{new_form, FormAction, State};
use crate::hooks::use_workspace_field;
use crate::layouts::DefaultLayout;
use crate::views::settings::{currency_options, timezone_options};
use crate::CustomersCache;
use api::customer::CustomerDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{
    HiOfficeBuilding, HiPencil, HiPlus, HiRefresh, HiSave, HiX,
};
use dioxus_free_icons::Icon;
use loom_core::{
    tenant::customer::{CreateCustomerInput, UpdateCustomerInput},
    validation::{validation_summary, Validate},
};

const PAGE_SIZE: usize = 15;

#[component]
pub fn Customers() -> Element {
    let cache: CustomersCache = use_context();
    let mut customers = use_signal(|| cache.read().clone());
    let mut loading = use_signal(|| cache.read().is_empty());
    let mut toasts: Toasts = use_context();
    let mut page = use_signal(|| 0_usize);

    // State machine drives the create-form lifecycle.
    let mut create_form = use_signal(new_form);

    // Create form fields — reactive defaults from workspace settings.
    let mut new_name = use_signal(String::new);
    let (new_currency, mut new_currency_set) = use_workspace_field(|ws| ws.currency.clone());
    let (new_timezone, mut new_timezone_set) = use_workspace_field(|ws| ws.timezone.clone());
    let mut new_comment = use_signal(String::new);
    let mut new_country = use_signal(String::new);

    // Inline edit state
    let mut editing_id = use_signal(|| Option::<String>::None);
    let mut edit_form = use_signal(new_form);
    let mut edit_name = use_signal(String::new);
    let mut edit_comment = use_signal(String::new);
    let mut edit_currency = use_signal(String::new);
    let mut edit_timezone = use_signal(String::new);
    let mut edit_country = use_signal(String::new);
    let mut edit_visible = use_signal(|| true);
    let mut edit_time_budget = use_signal(String::new);
    let mut edit_money_budget = use_signal(String::new);
    let mut edit_budget_monthly = use_signal(|| false);

    use_resource(move || async move {
        match api::customer::list_customers().await {
            Ok(list) => {
                customers.set(list);
                loading.set(false);
            }
            Err(e) => {
                toasts.push_error(e.to_string());
                loading.set(false);
            }
        }
    });

    let on_create = move |_| async move {
        let name = new_name.peek().clone();
        let currency = new_currency.peek().clone();
        let timezone = new_timezone.peek().clone();

        create_form.write().handle(&FormAction::Submit);
        let input = CreateCustomerInput {
            name: name.clone(),
            currency: currency.clone(),
            timezone: timezone.clone(),
        };
        if let Err(e) = input.validate() {
            create_form
                .write()
                .handle(&FormAction::Fail(validation_summary(&e)));
            return;
        }
        match api::customer::create_customer(name, currency, timezone).await {
            Ok(dto) => {
                customers.write().push(dto);
                new_name.set(String::new());
                new_comment.set(String::new());
                new_country.set(String::new());
                new_currency_set.set(None);
                new_timezone_set.set(None);
                create_form
                    .write()
                    .handle(&FormAction::Succeed("Customer created".into()));
                toasts.push_success("Customer created");
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
        let currency = edit_currency.peek().clone();
        let timezone = edit_timezone.peek().clone();
        let country = {
            let s = edit_country.peek().clone();
            if s.is_empty() { None } else { Some(s) }
        };
        let visible = *edit_visible.peek();

        edit_form.write().handle(&FormAction::Submit);
        let input = UpdateCustomerInput {
            name: name.clone(),
            currency: currency.clone(),
            timezone: timezone.clone(),
        };
        if let Err(e) = input.validate() {
            edit_form
                .write()
                .handle(&FormAction::Fail(validation_summary(&e)));
            return;
        }

        if let Err(e) = api::customer::update_customer(
            id.clone(),
            name.clone(),
            comment.clone(),
            currency.clone(),
            timezone.clone(),
            country.clone(),
            visible,
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
            api::customer::set_customer_budget(id.clone(), time_budget, money_budget, budget_monthly)
                .await
        {
            edit_form.write().handle(&FormAction::Fail(e.to_string()));
            toasts.push_error(e.to_string());
            return;
        }

        let updated = api::customer::CustomerDto {
            id: id.clone(),
            name,
            comment,
            currency,
            timezone,
            country,
            visible,
            time_budget,
            money_budget,
            budget_is_monthly: budget_monthly,
        };
        if let Some(item) = customers.write().iter_mut().find(|x| x.id == id) {
            *item = updated;
        }
        edit_form
            .write()
            .handle(&FormAction::Succeed("Customer saved".into()));
        editing_id.set(None);
        toasts.push_success("Customer saved");
    };

    let create_submitting = matches!(create_form.read().state(), State::Submitting {});

    // Pagination slice
    let total = customers.read().len();
    let current_page = *page.read();
    let page_items: Vec<CustomerDto> = customers
        .read()
        .iter()
        .skip(current_page * PAGE_SIZE)
        .take(PAGE_SIZE)
        .cloned()
        .collect();

    let columns = vec![
        ColumnDef::new("Name"),
        ColumnDef::new("Currency / Timezone"),
        ColumnDef::new("Budget"),
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
                                Icon { icon: HiOfficeBuilding, width: 18, height: 18 }
                                "New Customer"
                            }
                        }
                    }
                    CardContent {
                        div { class: "grid grid-cols-1 gap-4 md:grid-cols-2",
                            div { class: "form-field",
                                label { class: "form-label", r#for: "c-name", "Name" }
                                Input {
                                    id: "c-name",
                                    placeholder: "Acme Corp",
                                    value: new_name.read().clone(),
                                    oninput: move |e: FormEvent| new_name.set(e.value()),
                                }
                            }
                            div { class: "form-field",
                                label { class: "form-label", "Currency" }
                                Select::<String> {
                                    options: currency_options(),
                                    value: Some(new_currency.read().clone()),
                                    on_change: move |v| new_currency_set.set(Some(v)),
                                    placeholder: "Select currency".to_string(),
                                }
                            }
                            div { class: "form-field",
                                label { class: "form-label", "Timezone" }
                                SearchableSelect::<String> {
                                    options: timezone_options(),
                                    value: Some(new_timezone.read().clone()),
                                    on_change: move |v| new_timezone_set.set(Some(v)),
                                    placeholder: "Select timezone".to_string(),
                                }
                            }
                            div { class: "form-field",
                                label { class: "form-label", r#for: "c-country", "Country" }
                                Input {
                                    id: "c-country",
                                    placeholder: "DE",
                                    value: new_country.read().clone(),
                                    oninput: move |e: FormEvent| new_country.set(e.value()),
                                }
                            }
                            div { class: "form-field md:col-span-2",
                                label { class: "form-label", r#for: "c-comment", "Comment" }
                                Input {
                                    id: "c-comment",
                                    placeholder: "Optional notes…",
                                    value: new_comment.read().clone(),
                                    oninput: move |e: FormEvent| new_comment.set(e.value()),
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

                // ── Customer list ────────────────────────────────────────────
                div { class: "island",
                    div { class: "island-header",
                        span { class: "island-title", "Customers" }
                    }
                    DataTable {
                        columns,
                        total,
                        page: current_page,
                        page_size: PAGE_SIZE,
                        loading: *loading.read(),
                        on_page_change: move |p| page.set(p),

                        for customer in page_items {
                            {
                                let c = customer.clone();
                                let cid = c.id.clone();
                                let is_editing = editing_id.read().as_deref() == Some(c.id.as_str());
                                let edit_submitting = matches!(edit_form.read().state(), State::Submitting {});

                                rsx! {
                                    TableRow { key: "{c.id}", muted: !c.visible,
                                        TableCell { span { class: "font-medium", "{c.name}" } }
                                        TableCell {
                                            div { class: "flex flex-col gap-0.5 text-sm",
                                                span { "{c.currency}" }
                                                span { class: "text-secondary text-xs", "{c.timezone}" }
                                                if let Some(ref country) = c.country {
                                                    span { class: "text-secondary text-xs", "{country}" }
                                                }
                                            }
                                        }
                                        TableCell {
                                            div { class: "flex flex-col gap-0.5 text-xs text-secondary",
                                                if let Some(tb) = c.time_budget {
                                                    span { {format!("{:.1}h time", tb as f64 / 3600.0)} }
                                                }
                                                if let Some(mb) = c.money_budget {
                                                    span { {format!("{:.0} {}", mb as f64 / 100.0, c.currency)} }
                                                }
                                                if !c.visible {
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
                                                        let cu = customers.read()
                                                            .iter()
                                                            .find(|x| x.id == cid)
                                                            .cloned();
                                                        if let Some(cu) = cu {
                                                            edit_name.set(cu.name.clone());
                                                            edit_comment.set(cu.comment.clone().unwrap_or_default());
                                                            edit_currency.set(cu.currency.clone());
                                                            edit_timezone.set(cu.timezone.clone());
                                                            edit_country.set(cu.country.clone().unwrap_or_default());
                                                            edit_visible.set(cu.visible);
                                                            edit_time_budget.set(cu.time_budget.map(|v| format!("{:.1}", v as f64 / 3600.0)).unwrap_or_default());
                                                            edit_money_budget.set(cu.money_budget.map(|v| format!("{:.2}", v as f64 / 100.0)).unwrap_or_default());
                                                            edit_budget_monthly.set(cu.budget_is_monthly);
                                                            edit_form.write().handle(&FormAction::Reset);
                                                            editing_id.set(Some(cu.id));
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
                                                    label { class: "form-label", r#for: "e-name", "Name" }
                                                    Input {
                                                        id: "e-name",
                                                        value: edit_name.read().clone(),
                                                        oninput: move |e: FormEvent| edit_name.set(e.value()),
                                                    }
                                                }
                                                div { class: "form-field",
                                                    label { class: "form-label", "Currency" }
                                                    Select::<String> {
                                                        options: currency_options(),
                                                        value: Some(edit_currency.read().clone()),
                                                        on_change: move |v| edit_currency.set(v),
                                                        placeholder: "Select currency".to_string(),
                                                    }
                                                }
                                                div { class: "form-field",
                                                    label { class: "form-label", "Timezone" }
                                                    SearchableSelect::<String> {
                                                        options: timezone_options(),
                                                        value: Some(edit_timezone.read().clone()),
                                                        on_change: move |v| edit_timezone.set(v),
                                                        placeholder: "Select timezone".to_string(),
                                                    }
                                                }
                                                div { class: "form-field",
                                                    label { class: "form-label", r#for: "e-country", "Country" }
                                                    Input {
                                                        id: "e-country",
                                                        placeholder: "DE",
                                                        value: edit_country.read().clone(),
                                                        oninput: move |e: FormEvent| edit_country.set(e.value()),
                                                    }
                                                }
                                                div { class: "form-field",
                                                    label { class: "form-label", r#for: "e-comment", "Comment" }
                                                    Input {
                                                        id: "e-comment",
                                                        placeholder: "Optional notes…",
                                                        value: edit_comment.read().clone(),
                                                        oninput: move |e: FormEvent| edit_comment.set(e.value()),
                                                    }
                                                }
                                                div { class: "form-field",
                                                    label { class: "form-label", r#for: "e-time-budget", "Time Budget (hours)" }
                                                    Input {
                                                        id: "e-time-budget",
                                                        placeholder: "e.g. 160",
                                                        value: edit_time_budget.read().clone(),
                                                        oninput: move |e: FormEvent| edit_time_budget.set(e.value()),
                                                    }
                                                }
                                                div { class: "form-field",
                                                    label { class: "form-label", r#for: "e-money-budget", "Money Budget" }
                                                    Input {
                                                        id: "e-money-budget",
                                                        placeholder: "e.g. 5000.00",
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
