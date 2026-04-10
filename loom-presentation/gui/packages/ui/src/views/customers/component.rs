use crate::components::atoms::{ColumnDef, DataTable, ToastExt, Toasts};
use crate::layouts::DefaultLayout;
use crate::views::customers::create_form::CustomerCreateForm;
use crate::views::customers::customer_row::CustomerRow;
use crate::CustomersCache;
use api::customer::CustomerDto;
use dioxus::prelude::*;

const PAGE_SIZE: usize = 15;

#[component]
pub fn Customers() -> Element {
    let cache: CustomersCache = use_context();
    let mut customers = use_signal(|| cache.read().clone());
    let mut loading = use_signal(|| cache.read().is_empty());
    let mut toasts: Toasts = use_context();
    let mut page = use_signal(|| 0_usize);
    let editing_id = use_signal(|| Option::<String>::None);

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
                CustomerCreateForm {
                    on_created: move |dto: CustomerDto| customers.write().push(dto),
                }

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
                            CustomerRow {
                                key: "{customer.id}",
                                customer,
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
