use crate::components::atoms::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::components::atoms::{
    Button, ColumnDef, DataTable, Input, Select, SelectOption, TableCell, TableExpandRow, TableRow,
    ToastExt, Toasts,
};
use crate::form_machine::{new_form, FormAction, State};
use crate::layouts::DefaultLayout;
use crate::{ActivitiesCache, CustomersCache, ProjectsCache};
use api::activity::ActivityDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{HiPencil, HiPlus, HiRefresh, HiSave, HiTag, HiX};
use dioxus_free_icons::Icon;
use loom_core::{
    tenant::activity::{CreateActivityInput, UpdateActivityInput},
    validation::{validation_summary, Validate},
};

const PAGE_SIZE: usize = 15;

#[component]
pub fn Activities() -> Element {
    let activities_cache: ActivitiesCache = use_context();
    let projects_cache: ProjectsCache = use_context();
    let customers_cache: CustomersCache = use_context();
    let mut activities = use_signal(|| activities_cache.read().clone());
    let mut projects = use_signal(|| projects_cache.read().clone());
    let mut customers = use_signal(|| customers_cache.read().clone());
    let mut loading = use_signal(|| activities_cache.read().is_empty());
    let mut toasts: Toasts = use_context();
    let mut page = use_signal(|| 0_usize);

    // State machine drives the create-form lifecycle.
    let mut create_form = use_signal(new_form);

    // Create form
    let mut new_name = use_signal(String::new);
    let mut new_comment = use_signal(String::new);
    let mut new_project_id = use_signal(|| Option::<String>::None);
    let mut new_billable = use_signal(|| true);

    // Inline edit state
    let mut editing_id = use_signal(|| Option::<String>::None);
    let mut edit_form = use_signal(new_form);
    let mut edit_name = use_signal(String::new);
    let mut edit_comment = use_signal(String::new);
    let mut edit_visible = use_signal(|| true);
    let mut edit_billable = use_signal(|| true);

    use_resource(move || async move {
        match api::activity::list_activities().await {
            Ok(list) => activities.set(list),
            Err(e) => toasts.push_error(e.to_string()),
        }
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
        let project_id = new_project_id.peek().clone();

        create_form.write().handle(&FormAction::Submit);
        if let Err(e) = (CreateActivityInput { name: name.clone() }).validate() {
            create_form
                .write()
                .handle(&FormAction::Fail(validation_summary(&e)));
            return;
        }
        match api::activity::create_activity(project_id, name).await {
            Ok(dto) => {
                activities.write().push(dto);
                new_name.set(String::new());
                new_comment.set(String::new());
                new_project_id.set(None);
                create_form
                    .write()
                    .handle(&FormAction::Succeed("Activity created".into()));
                toasts.push_success("Activity created");
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

    let create_submitting = matches!(create_form.read().state(), State::Submitting {});

    // Pagination slice
    let total = activities.read().len();
    let current_page = *page.read();
    let page_items: Vec<ActivityDto> = activities
        .read()
        .iter()
        .skip(current_page * PAGE_SIZE)
        .take(PAGE_SIZE)
        .cloned()
        .collect();

    let columns = vec![
        ColumnDef::new("Name"),
        ColumnDef::new("Project"),
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
                                Icon { icon: HiTag, width: 18, height: 18 }
                                "New Activity"
                            }
                        }
                    }
                    CardContent {
                        div { class: "grid grid-cols-1 gap-4 md:grid-cols-2",
                            div { class: "form-field",
                                label { class: "form-label", r#for: "a-name", "Name" }
                                Input {
                                    id: "a-name",
                                    placeholder: "Development",
                                    value: new_name.read().clone(),
                                    oninput: move |e: FormEvent| new_name.set(e.value()),
                                }
                            }
                            div { class: "form-field",
                                label { class: "form-label", "Project (optional)" }
                                Select::<String> {
                                    options: {
                                        let mut opts = vec![SelectOption::new("".to_string(), "— Global —".to_string())];
                                        opts.extend(projects.read().iter().map(|p| {
                                    let customer_name = customers.read()
                                        .iter()
                                        .find(|c| c.id == p.customer_id)
                                        .map(|c| c.name.clone());
                                    let mut opt = SelectOption::new(p.id.clone(), p.name.clone());
                                    if let Some(cn) = customer_name { opt = opt.sublabel(cn); }
                                    opt
                                }));
                                        opts
                                    },
                                    value: new_project_id.read().clone(),
                                    on_change: move |id: String| {
                                        new_project_id.set(if id.is_empty() { None } else { Some(id) })
                                    },
                                    placeholder: "Global activity…".to_string(),
                                }
                            }
                            div { class: "form-field md:col-span-2",
                                label { class: "form-label", r#for: "a-comment", "Comment" }
                                Input {
                                    id: "a-comment",
                                    placeholder: "Optional description…",
                                    value: new_comment.read().clone(),
                                    oninput: move |e: FormEvent| new_comment.set(e.value()),
                                }
                            }
                            div { class: "form-field flex items-center gap-3",
                                label { class: "form-label", "Billable by default" }
                                input {
                                    r#type: "checkbox",
                                    class: "form-checkbox",
                                    checked: *new_billable.read(),
                                    oninput: move |_| { let v = *new_billable.peek(); new_billable.set(!v); },
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

                // ── Activity list ────────────────────────────────────────────
                div { class: "island",
                    div { class: "island-header",
                        span { class: "island-title", "Activities" }
                    }
                    DataTable {
                        columns,
                        total,
                        page: current_page,
                        page_size: PAGE_SIZE,
                        loading: *loading.read(),
                        on_page_change: move |p| page.set(p),

                        for activity in page_items {
                            {
                                let a = activity.clone();
                                let aid = a.id.clone();
                                let is_editing = editing_id.read().as_deref() == Some(a.id.as_str());
                                let project_name = a.project_id.as_ref().and_then(|pid| {
                                    projects.read().iter().find(|p| &p.id == pid).map(|p| p.name.clone())
                                });
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
                                        TableExpandRow { col_count,
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
                        }
                    }
                }
            }
        }
    }
}
