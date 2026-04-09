use crate::components::atoms::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::components::atoms::{Button, Input, Select, SelectOption, ToastMessage, Toasts};
use crate::layouts::DefaultLayout;
use api::activity::ActivityDto;
use api::project::ProjectDto;
use api::timesheet::TimesheetDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{HiClock, HiPlay, HiStop};
use dioxus_free_icons::Icon;

#[component]
pub fn Timesheets() -> Element {
    let mut timesheets = use_signal(Vec::<TimesheetDto>::new);
    let mut running = use_signal(|| Option::<TimesheetDto>::None);
    let mut projects = use_signal(Vec::<ProjectDto>::new);
    let mut activities = use_signal(Vec::<ActivityDto>::new);
    let mut project_id = use_signal(|| Option::<String>::None);
    let mut activity_id = use_signal(|| Option::<String>::None);
    let mut description = use_signal(String::new);
    let mut toasts: Toasts = use_context();

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
        if let Ok(list) = api::project::list_projects().await {
            projects.set(list);
        }
        if let Ok(list) = api::activity::list_activities().await {
            activities.set(list);
        }
    });

    let on_start = move |_| async move {
        let pid = project_id.peek().clone();
        let aid = activity_id.peek().clone();
        let desc = description.peek().clone();
        let (Some(pid), Some(aid)) = (pid, aid) else {
            return;
        };
        let desc_opt = if desc.is_empty() { None } else { Some(desc) };
        match api::timesheet::start_timesheet(pid, aid, desc_opt, true).await {
            Ok(dto) => {
                running.set(Some(dto));
                project_id.set(None);
                activity_id.set(None);
                description.set(String::new());
            }
            Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
        }
    };

    let on_stop = move |_| async move {
        let maybe_ts = running.peek().clone();
        if let Some(ts) = maybe_ts {
            match api::timesheet::stop_timesheet(ts.id).await {
                Ok(()) => reload().await,
                Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
            }
        }
    };

    rsx! {
        DefaultLayout {
            div { class: "space-y-6",

                // Running timer
                match running.read().as_ref() {
                    Some(ts) => rsx! {
                        Card { data_size: "md",
                            CardHeader {
                                CardTitle {
                                    div { class: "flex items-center gap-2",
                                        Icon { icon: HiClock, width: 18, height: 18 }
                                        "Timer"
                                    }
                                }
                            }
                            CardContent {
                                div { class: "flex flex-col gap-2",
                                    p { class: "text-sm font-medium",
                                        {
                                            let proj_name = projects.read()
                                                .iter()
                                                .find(|p| p.id == ts.project_id)
                                                .map(|p| p.name.clone())
                                                .unwrap_or_else(|| ts.project_id.clone());
                                            let act_name = activities.read()
                                                .iter()
                                                .find(|a| a.id == ts.activity_id)
                                                .map(|a| a.name.clone())
                                                .unwrap_or_else(|| ts.activity_id.clone());
                                            rsx! { "{proj_name} / {act_name}" }
                                        }
                                    }
                                    if let Some(desc) = &ts.description {
                                        p { class: "text-sm text-muted-foreground", "{desc}" }
                                    }
                                    p { class: "text-xs text-muted-foreground", "Started: {ts.start_time}" }
                                }
                            }
                            CardFooter {
                                Button { onclick: on_stop,
                                    Icon { icon: HiStop, width: 16, height: 16 }
                                    "Stop"
                                }
                            }
                        }
                    },
                    None => rsx! {
                        Card { data_size: "md",
                            CardHeader {
                                CardTitle {
                                    div { class: "flex items-center gap-2",
                                        Icon { icon: HiClock, width: 18, height: 18 }
                                        "Timer"
                                    }
                                }
                            }
                            CardContent {
                                div { class: "flex flex-col gap-4",
                                    div { class: "form-field",
                                        label { class: "form-label", "Project" }
                                        Select::<String> {
                                            options: projects.read().iter()
                                                .map(|p| SelectOption::new(p.id.clone(), p.name.clone()))
                                                .collect(),
                                            value: project_id.read().clone(),
                                            on_change: move |id: String| project_id.set(Some(id)),
                                            placeholder: "Select project…".to_string(),
                                        }
                                    }
                                    div { class: "form-field",
                                        label { class: "form-label", "Activity" }
                                        Select::<String> {
                                            options: activities.read().iter()
                                                .map(|a| SelectOption::new(a.id.clone(), a.name.clone()))
                                                .collect(),
                                            value: activity_id.read().clone(),
                                            on_change: move |id: String| activity_id.set(Some(id)),
                                            placeholder: "Select activity…".to_string(),
                                        }
                                    }
                                    div { class: "form-field",
                                        label { class: "form-label", r#for: "ts-description", "Description" }
                                        Input {
                                            id: "ts-description",
                                            placeholder: "Optional notes…",
                                            value: description.read().clone(),
                                            oninput: move |e: FormEvent| description.set(e.value()),
                                        }
                                    }
                                }
                            }
                            CardFooter {
                                Button { onclick: on_start,
                                    Icon { icon: HiPlay, width: 16, height: 16 }
                                    "Start"
                                }
                            }
                        }
                    },
                }

                // Recent timesheets
                h2 { class: "text-base font-semibold text-muted-foreground", "Recent" }
                div { class: "flex flex-col gap-3",
                    for ts in timesheets.read().iter() {
                        {
                            let t = ts.clone();
                            let proj_name = projects.read()
                                .iter()
                                .find(|p| p.id == t.project_id)
                                .map(|p| p.name.clone())
                                .unwrap_or_else(|| t.project_id.clone());
                            let act_name = activities.read()
                                .iter()
                                .find(|a| a.id == t.activity_id)
                                .map(|a| a.name.clone())
                                .unwrap_or_else(|| t.activity_id.clone());
                            rsx! {
                                Card { key: "{t.id}",
                                    CardHeader {
                                        CardTitle { "{proj_name} / {act_name}" }
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
