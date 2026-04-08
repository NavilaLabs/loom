use crate::components::atoms::{Button, Input};
use crate::components::atoms::card::{Card, CardContent, CardHeader, CardTitle};
use crate::layouts::DefaultLayout;
use api::customer::CustomerDto;
use dioxus::prelude::*;

#[component]
pub fn Customers() -> Element {
    let mut customers = use_signal(Vec::<CustomerDto>::new);
    let mut name = use_signal(String::new);
    let mut currency = use_signal(|| "EUR".to_string());
    let mut timezone = use_signal(|| "UTC".to_string());
    let mut error = use_signal(|| Option::<String>::None);

    // Load on mount
    use_resource(move || async move {
        match api::customer::list_customers().await {
            Ok(list) => customers.set(list),
            Err(e) => error.set(Some(e.to_string())),
        }
    });

    let on_create = move |_| async move {
        let n = name.peek().clone();
        let c = currency.peek().clone();
        let tz = timezone.peek().clone();
        if n.is_empty() {
            return;
        }
        match api::customer::create_customer(n, c, tz).await {
            Ok(dto) => {
                customers.write().push(dto);
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
                    h2 { class: "text-lg font-semibold", "New Customer" }
                    Input {
                        placeholder: "Name",
                        value: name.read().clone(),
                        oninput: move |e: FormEvent| name.set(e.value()),
                    }
                    Input {
                        placeholder: "Currency (e.g. EUR)",
                        value: currency.read().clone(),
                        oninput: move |e: FormEvent| currency.set(e.value()),
                    }
                    Input {
                        placeholder: "Timezone (e.g. UTC)",
                        value: timezone.read().clone(),
                        oninput: move |e: FormEvent| timezone.set(e.value()),
                    }
                    Button { onclick: on_create, "Create" }
                    if let Some(err) = error.read().as_ref() {
                        p { class: "text-destructive text-sm", "{err}" }
                    }
                }

                // Customer list
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    for customer in customers.read().iter() {
                        {
                            let c = customer.clone();
                            rsx! {
                                Card { key: "{c.id}",
                                    CardHeader {
                                        CardTitle { "{c.name}" }
                                    }
                                    CardContent {
                                        p { class: "text-sm text-muted-foreground",
                                            "Currency: {c.currency} | TZ: {c.timezone}"
                                        }
                                        if !c.visible {
                                            p { class: "text-xs text-muted-foreground", "(hidden)" }
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
