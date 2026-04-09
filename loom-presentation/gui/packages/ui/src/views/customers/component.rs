use crate::components::atoms::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::components::atoms::{Button, Input, ToastMessage, Toasts};
use crate::layouts::DefaultLayout;
use api::customer::CustomerDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{HiOfficeBuilding, HiPlus};
use dioxus_free_icons::Icon;

#[component]
pub fn Customers() -> Element {
    let mut customers = use_signal(Vec::<CustomerDto>::new);
    let mut name = use_signal(String::new);
    let mut currency = use_signal(|| "EUR".to_string());
    let mut timezone = use_signal(|| "UTC".to_string());
    let mut toasts: Toasts = use_context();

    use_resource(move || async move {
        match api::customer::list_customers().await {
            Ok(list) => customers.set(list),
            Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
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
                toasts.write().push(ToastMessage::success("Customer created"));
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
                                Icon { icon: HiOfficeBuilding, width: 18, height: 18 }
                                "New Customer"
                            }
                        }
                    }
                    CardContent {
                        div { class: "flex flex-col gap-4",
                            div { class: "form-field",
                                label { class: "form-label", r#for: "customer-name", "Name" }
                                Input {
                                    id: "customer-name",
                                    placeholder: "Acme Corp",
                                    value: name.read().clone(),
                                    oninput: move |e: FormEvent| name.set(e.value()),
                                }
                            }
                            div { class: "form-field",
                                label { class: "form-label", r#for: "customer-currency", "Currency" }
                                Input {
                                    id: "customer-currency",
                                    placeholder: "EUR",
                                    value: currency.read().clone(),
                                    oninput: move |e: FormEvent| currency.set(e.value()),
                                }
                            }
                            div { class: "form-field",
                                label { class: "form-label", r#for: "customer-timezone", "Timezone" }
                                Input {
                                    id: "customer-timezone",
                                    placeholder: "UTC",
                                    value: timezone.read().clone(),
                                    oninput: move |e: FormEvent| timezone.set(e.value()),
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
