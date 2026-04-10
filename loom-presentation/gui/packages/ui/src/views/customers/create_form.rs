use crate::components::atoms::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::components::atoms::{Button, Input, SearchableSelect, Select, ToastExt, Toasts};
use crate::form_machine::{new_form, FormAction, State};
use crate::hooks::use_workspace_field;
use crate::views::settings::{currency_options, timezone_options};
use api::customer::CustomerDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{HiOfficeBuilding, HiPlus, HiRefresh};
use dioxus_free_icons::Icon;
use loom_core::{
    tenant::customer::CreateCustomerInput,
    validation::{validation_summary, Validate},
};

#[derive(Clone, PartialEq, Props)]
pub(super) struct CustomerCreateFormProps {
    pub on_created: EventHandler<CustomerDto>,
}

#[component]
pub(super) fn CustomerCreateForm(props: CustomerCreateFormProps) -> Element {
    let mut toasts: Toasts = use_context();

    let mut create_form = use_signal(new_form);
    let mut new_name = use_signal(String::new);
    let (new_currency, mut new_currency_set) = use_workspace_field(|ws| ws.currency.clone());
    let (new_timezone, mut new_timezone_set) = use_workspace_field(|ws| ws.timezone.clone());
    let mut new_comment = use_signal(String::new);
    let mut new_country = use_signal(String::new);

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
                new_name.set(String::new());
                new_comment.set(String::new());
                new_country.set(String::new());
                new_currency_set.set(None);
                new_timezone_set.set(None);
                create_form
                    .write()
                    .handle(&FormAction::Succeed("Customer created".into()));
                toasts.push_success("Customer created");
                props.on_created.call(dto);
            }
            Err(e) => {
                create_form.write().handle(&FormAction::Fail(e.to_string()));
                toasts.push_error(e.to_string());
            }
        }
    };

    let create_submitting = matches!(create_form.read().state(), State::Submitting {});

    rsx! {
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
    }
}
