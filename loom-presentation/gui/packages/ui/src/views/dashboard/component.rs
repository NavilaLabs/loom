use crate::components::atoms::card::{Card, CardContent};
use crate::components::atoms::{Button, ButtonVariant, Select, SelectOption, ToastMessage, Toasts};
use crate::layouts::DefaultLayout;
use api::activity::ActivityDto;
use api::project::ProjectDto;
use api::timesheet::TimesheetDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{HiLightningBolt, HiPlay, HiStop};
use dioxus_free_icons::Icon;

#[component]
pub fn Dashboard() -> Element {
    let mut running: crate::RunningTimer = use_context();
    let mut toasts: Toasts = use_context();

    let mut projects = use_signal(Vec::<ProjectDto>::new);
    let mut activities = use_signal(Vec::<ActivityDto>::new);
    let mut recent = use_signal(Vec::<TimesheetDto>::new);
    let mut selected_project_id = use_signal(|| Option::<String>::None);
    let mut selected_activity_id = use_signal(|| Option::<String>::None);
    let elapsed_secs: crate::RunningElapsed = use_context();

    use_resource(move || async move {
        if let Ok(list) = api::project::list_projects().await {
            projects.set(list);
        }
        if let Ok(list) = api::activity::list_activities().await {
            activities.set(list);
        }
        if let Ok(list) = api::timesheet::list_timesheets().await {
            recent.set(list);
        }
    });

    let on_start = move |_| async move {
        let pid = selected_project_id.peek().clone();
        let aid = selected_activity_id.peek().clone();
        match api::timesheet::start_timesheet(pid, aid, None, true).await {
            Ok(dto) => {
                running.set(Some(dto));
                selected_project_id.set(None);
                selected_activity_id.set(None);
            }
            Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
        }
    };

    let on_stop = move |_| async move {
        let ts_id = running.peek().as_ref().map(|ts| ts.id.clone());
        if let Some(id) = ts_id {
            match api::timesheet::stop_timesheet(id).await {
                Ok(()) => {
                    running.set(None);
                    if let Ok(list) = api::timesheet::list_timesheets().await {
                        recent.set(list);
                    }
                }
                Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
            }
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        DefaultLayout {
            div { class: "space-y-6",

                // ── Quick Start / Running Timer ──────────────────────────────
                match running.read().clone() {
                    Some(ts) => {
                        let proj_name = ts.project_id.as_ref()
                            .and_then(|pid| projects.read().iter().find(|p| &p.id == pid).map(|p| p.name.clone()))
                            .unwrap_or_else(|| "Unassigned".to_string());
                        let act_name = ts.activity_id.as_ref()
                            .and_then(|aid| activities.read().iter().find(|a| &a.id == aid).map(|a| a.name.clone()))
                            .unwrap_or_else(|| "Unassigned".to_string());
                        let e = *elapsed_secs.read();
                        rsx! {
                            Card { data_size: "md",
                                CardContent {
                                    div { class: "dashboard-timer",
                                        // Status row
                                        div { class: "dashboard-timer-header",
                                            div { class: "dashboard-timer-status",
                                                span { class: "timer-dot" }
                                                span { class: "dashboard-timer-status-label",
                                                    "Timer Running"
                                                }
                                            }
                                            Button {
                                                variant: ButtonVariant::Ghost,
                                                onclick: on_stop,
                                                Icon { icon: HiStop, width: 14, height: 14 }
                                                "Stop"
                                            }
                                        }
                                        // Large elapsed time
                                        div { class: "dashboard-timer-time",
                                            span { class: "dashboard-timer-elapsed",
                                                { format!("{:02}:{:02}:{:02}", e / 3600, (e % 3600) / 60, e % 60) }
                                            }
                                        }
                                        // Context
                                        div { class: "dashboard-timer-meta",
                                            span { class: "text-sm font-medium",
                                                "{proj_name} · {act_name}"
                                            }
                                            if let Some(ref desc) = ts.description {
                                                span { class: "text-xs text-secondary", "{desc}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    None => rsx! {
                        Card { data_size: "md",
                            CardContent {
                                div { class: "qs-card",
                                    div { class: "qs-header",
                                        span { class: "qs-label", "Quick Start" }
                                        Icon { icon: HiLightningBolt, width: 14, height: 14 }
                                    }
                                    Select::<String> {
                                        options: projects.read().iter()
                                            .map(|p| SelectOption::new(p.id.clone(), p.name.clone()))
                                            .collect(),
                                        value: selected_project_id.read().clone(),
                                        on_change: move |id: String| selected_project_id.set(Some(id)),
                                        placeholder: "Select project…".to_string(),
                                    }
                                    Select::<String> {
                                        options: activities.read().iter()
                                            .map(|a| SelectOption::new(a.id.clone(), a.name.clone()))
                                            .collect(),
                                        value: selected_activity_id.read().clone(),
                                        on_change: move |id: String| selected_activity_id.set(Some(id)),
                                        placeholder: "Select activity…".to_string(),
                                    }
                                    Button {
                                        onclick: on_start,
                                        class: "qs-start-btn",
                                        Icon { icon: HiPlay, width: 16, height: 16 }
                                        "Start Session"
                                    }
                                }
                            }
                        }
                    },
                }

                // ── Recent Entries ───────────────────────────────────────────
                if !recent.read().is_empty() {
                    div { class: "flex flex-col gap-2",
                        h2 { class: "text-base font-semibold", "Recent Entries" }
                        div { class: "flex flex-col gap-2",
                            for ts in recent.read().iter().take(5) {
                                {
                                    let proj_name = ts.project_id.as_ref()
                                        .and_then(|pid| projects.read().iter().find(|p| &p.id == pid).map(|p| p.name.clone()))
                                        .unwrap_or_else(|| "Unassigned".to_string());
                                    let act_name = ts.activity_id.as_ref()
                                        .and_then(|aid| activities.read().iter().find(|a| &a.id == aid).map(|a| a.name.clone()))
                                        .unwrap_or_else(|| "Unassigned".to_string());
                                    let duration_str = ts.duration.map(|d| {
                                        let h = d / 3600;
                                        let m = (d % 3600) / 60;
                                        if h > 0 { format!("{h}h {m:02}m") } else { format!("{m}m") }
                                    });
                                    rsx! {
                                        Card { key: "{ts.id}",
                                            CardContent {
                                                div { class: "flex items-center justify-between",
                                                    div { class: "flex flex-col gap-1",
                                                        span { class: "font-medium text-sm",
                                                            if let Some(ref desc) = ts.description {
                                                                "{desc}"
                                                            } else {
                                                                "{proj_name} / {act_name}"
                                                            }
                                                        }
                                                        span { class: "text-xs text-secondary",
                                                            "{proj_name} · {act_name}"
                                                        }
                                                    }
                                                    if let Some(ref d) = duration_str {
                                                        span { class: "text-sm font-medium", "{d}" }
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
}
