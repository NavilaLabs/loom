use crate::components::atoms::{Button, Input};
use crate::components::atoms::card::{Card, CardContent, CardHeader, CardTitle};
use crate::layouts::DefaultLayout;
use api::timesheet::TimesheetDto;
use dioxus::prelude::*;

#[component]
pub fn Timesheets() -> Element {
    let mut timesheets = use_signal(Vec::<TimesheetDto>::new);
    let mut running = use_signal(|| Option::<TimesheetDto>::None);
    let mut project_id = use_signal(String::new);
    let mut activity_id = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut error = use_signal(|| Option::<String>::None);

    let reload = move || async move {
        if let Ok(list) = api::timesheet::list_timesheets().await {
            timesheets.set(list);
        }
        if let Ok(r) = api::timesheet::running_timesheet().await {
            running.set(r);
        }
    };

    use_resource(move || async move {
        if let Ok(list) = api::timesheet::list_timesheets().await {
            timesheets.set(list);
        }
        if let Ok(r) = api::timesheet::running_timesheet().await {
            running.set(r);
        }
    });

    let on_start = move |_| async move {
        let pid = project_id.peek().clone();
        let aid = activity_id.peek().clone();
        let desc = description.peek().clone();
        if pid.is_empty() || aid.is_empty() {
            return;
        }
        let desc_opt = if desc.is_empty() { None } else { Some(desc) };
        match api::timesheet::start_timesheet(pid, aid, desc_opt, true).await {
            Ok(dto) => {
                running.set(Some(dto));
                project_id.set(String::new());
                activity_id.set(String::new());
                description.set(String::new());
            }
            Err(e) => error.set(Some(e.to_string())),
        }
    };

    let on_stop = move |_| async move {
        if let Some(ts) = running.peek().clone() {
            match api::timesheet::stop_timesheet(ts.id).await {
                Ok(()) => reload().await,
                Err(e) => error.set(Some(e.to_string())),
            }
        }
    };

    rsx! {
        DefaultLayout {
            div { class: "space-y-6",

                // Running timer
                div { class: "p-4 border rounded-md space-y-2",
                    h2 { class: "text-lg font-semibold", "Timer" }
                    match running.read().as_ref() {
                        Some(ts) => rsx! {
                            div { class: "flex flex-col gap-1",
                                p { class: "text-sm",
                                    "Running: {ts.project_id} / {ts.activity_id}"
                                }
                                if let Some(desc) = &ts.description {
                                    p { class: "text-sm text-muted-foreground", "{desc}" }
                                }
                                p { class: "text-xs text-muted-foreground", "Started: {ts.start_time}" }
                                Button { onclick: on_stop, "Stop" }
                            }
                        },
                        None => rsx! {
                            div { class: "flex flex-col gap-2",
                                Input {
                                    placeholder: "Project ID",
                                    value: project_id.read().clone(),
                                    oninput: move |e: FormEvent| project_id.set(e.value()),
                                }
                                Input {
                                    placeholder: "Activity ID",
                                    value: activity_id.read().clone(),
                                    oninput: move |e: FormEvent| activity_id.set(e.value()),
                                }
                                Input {
                                    placeholder: "Description (optional)",
                                    value: description.read().clone(),
                                    oninput: move |e: FormEvent| description.set(e.value()),
                                }
                                Button { onclick: on_start, "Start" }
                            }
                        },
                    }
                    if let Some(err) = error.read().as_ref() {
                        p { class: "text-destructive text-sm", "{err}" }
                    }
                }

                // Recent timesheets
                h2 { class: "text-lg font-semibold", "Recent" }
                div { class: "flex flex-col gap-3",
                    for ts in timesheets.read().iter() {
                        {
                            let t = ts.clone();
                            rsx! {
                                Card { key: "{t.id}",
                                    CardHeader {
                                        CardTitle {
                                            "{t.project_id} / {t.activity_id}"
                                        }
                                    }
                                    CardContent {
                                        if let Some(desc) = &t.description {
                                            p { class: "text-sm", "{desc}" }
                                        }
                                        p { class: "text-xs text-muted-foreground",
                                            "Start: {t.start_time}"
                                        }
                                        if let Some(dur) = t.duration {
                                            p { class: "text-xs text-muted-foreground",
                                                "Duration: {dur}s"
                                            }
                                        }
                                        if t.billable {
                                            p { class: "text-xs text-green-600", "Billable" }
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
