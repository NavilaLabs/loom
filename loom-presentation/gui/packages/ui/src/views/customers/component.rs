use crate::components::atoms::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::components::atoms::{Button, Input, ToastMessage, Toasts};
use crate::layouts::DefaultLayout;
use api::customer::CustomerDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{
    HiOfficeBuilding, HiPencil, HiPlus, HiSave, HiX,
};
use dioxus_free_icons::Icon;

#[component]
pub fn Customers() -> Element {
    let mut customers = use_signal(Vec::<CustomerDto>::new);
    let mut toasts: Toasts = use_context();

    // Create form fields
    let mut new_name = use_signal(String::new);
    let mut new_currency = use_signal(|| "EUR".to_string());
    let mut new_timezone = use_signal(|| "UTC".to_string());
    let mut new_comment = use_signal(String::new);
    let mut new_country = use_signal(String::new);

    // Inline edit state
    let mut editing_id = use_signal(|| Option::<String>::None);
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
            Ok(list) => customers.set(list),
            Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
        }
    });

    let on_create = move |_| async move {
        let name = new_name.peek().clone();
        if name.is_empty() {
            return;
        }
        let currency = new_currency.peek().clone();
        let timezone = new_timezone.peek().clone();
        match api::customer::create_customer(name, currency, timezone).await {
            Ok(dto) => {
                customers.write().push(dto);
                new_name.set(String::new());
                new_comment.set(String::new());
                new_country.set(String::new());
                toasts.write().push(ToastMessage::success("Customer created"));
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
        let currency = edit_currency.peek().clone();
        let timezone = edit_timezone.peek().clone();
        let country = {
            let s = edit_country.peek().clone();
            if s.is_empty() { None } else { Some(s) }
        };
        let visible = *edit_visible.peek();

        // Update basic info
        if let Err(e) = api::customer::update_customer(
            id.clone(), name.clone(), comment.clone(), currency.clone(),
            timezone.clone(), country.clone(), visible,
        ).await {
            toasts.write().push(ToastMessage::error(e.to_string()));
            return;
        }

        // Update budget
        // time_budget stored in seconds; GUI shows hours
        let time_budget: Option<i32> = edit_time_budget.peek().parse::<f64>().ok()
            .map(|h| (h * 3600.0) as i32);
        let money_budget: Option<i64> = edit_money_budget.peek().parse::<f64>().ok()
            .map(|v| (v * 100.0) as i64);
        let budget_monthly = *edit_budget_monthly.peek();
        if let Err(e) = api::customer::set_customer_budget(
            id.clone(), time_budget, money_budget, budget_monthly,
        ).await {
            toasts.write().push(ToastMessage::error(e.to_string()));
            return;
        }

        // Patch the item in-place — no extra round-trip needed
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
        editing_id.set(None);
        toasts.write().push(ToastMessage::success("Customer saved"));
    };

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
                                label { class: "form-label", r#for: "c-currency", "Currency" }
                                Input {
                                    id: "c-currency",
                                    placeholder: "EUR",
                                    value: new_currency.read().clone(),
                                    oninput: move |e: FormEvent| new_currency.set(e.value()),
                                }
                            }
                            div { class: "form-field",
                                label { class: "form-label", r#for: "c-tz", "Timezone" }
                                Input {
                                    id: "c-tz",
                                    placeholder: "UTC",
                                    value: new_timezone.read().clone(),
                                    oninput: move |e: FormEvent| new_timezone.set(e.value()),
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
                    }
                    CardFooter {
                        Button { onclick: on_create,
                            Icon { icon: HiPlus, width: 16, height: 16 }
                            "Create"
                        }
                    }
                }

                // ── Customer list ────────────────────────────────────────────
                div { class: "flex flex-col gap-3",
                    for customer in customers.read().clone() {
                        {
                            let c = customer.clone();
                            let is_editing = editing_id.read().as_deref() == Some(c.id.as_str());
                            let cid = c.id.clone();

                            if is_editing {
                                rsx! {
                                    Card { key: "{c.id}",
                                        CardHeader {
                                            CardTitle {
                                                div { class: "flex items-center justify-between",
                                                    span { "{c.name}" }
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
                                                    label { class: "form-label", r#for: "e-name", "Name" }
                                                    Input {
                                                        id: "e-name",
                                                        value: edit_name.read().clone(),
                                                        oninput: move |e: FormEvent| edit_name.set(e.value()),
                                                    }
                                                }
                                                div { class: "form-field",
                                                    label { class: "form-label", r#for: "e-currency", "Currency" }
                                                    Input {
                                                        id: "e-currency",
                                                        value: edit_currency.read().clone(),
                                                        oninput: move |e: FormEvent| edit_currency.set(e.value()),
                                                    }
                                                }
                                                div { class: "form-field",
                                                    label { class: "form-label", r#for: "e-tz", "Timezone" }
                                                    Input {
                                                        id: "e-tz",
                                                        value: edit_timezone.read().clone(),
                                                        oninput: move |e: FormEvent| edit_timezone.set(e.value()),
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
                                                div { class: "form-field md:col-span-2",
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
                                                    label { class: "form-label", r#for: "e-money-budget", "Money Budget (EUR)" }
                                                    Input {
                                                        id: "e-money-budget",
                                                        placeholder: "e.g. 5000.00",
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
                                    Card { key: "{c.id}",
                                        CardContent {
                                            div { class: "flex items-center justify-between",
                                                div { class: "flex flex-col gap-1",
                                                    span { class: "font-medium", "{c.name}" }
                                                    span { class: "text-sm text-secondary",
                                                        "{c.currency} · {c.timezone}"
                                                        if let Some(country) = &c.country {
                                                            " · {country}"
                                                        }
                                                    }
                                                    div { class: "flex gap-3 text-xs text-secondary",
                                                        if let Some(tb) = c.time_budget {
                                                            span { {format!("Time: {:.1}h", tb as f64 / 3600.0)} }
                                                        }
                                                        if let Some(mb) = c.money_budget {
                                                            span { "Budget: {mb / 100} {c.currency}" }
                                                        }
                                                        if !c.visible {
                                                            span { class: "text-warning", "Hidden" }
                                                        }
                                                    }
                                                }
                                                Button {
                                                    onclick: move |_| {
                                                        let customer = customers.read()
                                                            .iter()
                                                            .find(|x| x.id == cid)
                                                            .cloned();
                                                        if let Some(cu) = customer {
                                                            edit_name.set(cu.name.clone());
                                                            edit_comment.set(cu.comment.clone().unwrap_or_default());
                                                            edit_currency.set(cu.currency.clone());
                                                            edit_timezone.set(cu.timezone.clone());
                                                            edit_country.set(cu.country.clone().unwrap_or_default());
                                                            edit_visible.set(cu.visible);
                                                            // convert seconds → hours for display
                                                            edit_time_budget.set(cu.time_budget.map(|v| format!("{:.1}", v as f64 / 3600.0)).unwrap_or_default());
                                                            edit_money_budget.set(cu.money_budget.map(|v| format!("{:.2}", v as f64 / 100.0)).unwrap_or_default());
                                                            edit_budget_monthly.set(cu.budget_is_monthly);
                                                            editing_id.set(Some(cu.id));
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
