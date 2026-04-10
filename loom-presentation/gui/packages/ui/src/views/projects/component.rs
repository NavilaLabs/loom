use crate::components::atoms::{ColumnDef, DataTable, ToastExt, Toasts};
use crate::layouts::DefaultLayout;
use crate::views::projects::create_form::ProjectCreateForm;
use crate::views::projects::project_row::ProjectRow;
use crate::{CustomersCache, ProjectsCache};
use api::project::ProjectDto;
use dioxus::prelude::*;

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
    let editing_id = use_signal(|| Option::<String>::None);

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
                ProjectCreateForm {
                    customers,
                    on_created: move |dto: ProjectDto| projects.write().push(dto),
                }

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
                            ProjectRow {
                                key: "{project.id}",
                                project,
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
