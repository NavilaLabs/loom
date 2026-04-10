use crate::components::atoms::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::components::atoms::{Button, Input, Select, SelectOption, ToastExt, Toasts};
use crate::form_machine::{new_form, FormAction, State};
use api::customer::CustomerDto;
use api::project::ProjectDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{HiBriefcase, HiPlus, HiRefresh};
use dioxus_free_icons::Icon;
use loom_core::{
    tenant::project::CreateProjectInput,
    validation::{validation_summary, Validate},
};

#[derive(Clone, PartialEq, Props)]
pub(super) struct ProjectCreateFormProps {
    pub customers: Signal<Vec<CustomerDto>>,
    pub on_created: EventHandler<ProjectDto>,
}

#[component]
pub(super) fn ProjectCreateForm(props: ProjectCreateFormProps) -> Element {
    let mut toasts: Toasts = use_context();
    let customers = props.customers;

    let mut create_form = use_signal(new_form);
    let mut new_name = use_signal(String::new);
    let mut new_customer_id = use_signal(|| Option::<String>::None);

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
                new_name.set(String::new());
                new_customer_id.set(None);
                create_form
                    .write()
                    .handle(&FormAction::Succeed("Project created".into()));
                toasts.push_success("Project created");
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
    }
}
