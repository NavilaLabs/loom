use crate::components::atoms::{ColumnDef, DataTable, ToastExt, Toasts};
use crate::layouts::DefaultLayout;
use crate::views::activities::activity_row::ActivityRow;
use crate::views::activities::create_form::ActivityCreateForm;
use crate::{ActivitiesCache, CustomersCache, ProjectsCache};
use api::activity::ActivityDto;
use dioxus::prelude::*;

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
    let editing_id = use_signal(|| Option::<String>::None);

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
                ActivityCreateForm {
                    projects,
                    customers,
                    on_created: move |dto: ActivityDto| activities.write().push(dto),
                }

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
                            ActivityRow {
                                key: "{activity.id}",
                                activity,
                                activities,
                                projects,
                                customers,
                                editing_id,
                                col_count,
                            }
                        }
                    }
                }
            }
        }
    }
}
