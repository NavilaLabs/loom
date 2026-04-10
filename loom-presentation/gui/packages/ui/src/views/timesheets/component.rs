use crate::components::atoms::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::components::atoms::{
    Button, ColumnDef, DataTable, Input, Select, SelectOption, TableCell, TableExpandRow, TableRow,
    ToastExt, Toasts,
};
use crate::formatting;
use crate::layouts::DefaultLayout;
use crate::{ActivitiesCache, ProjectsCache, TagsCache, TimesheetsCache};
use api::tag::TagDto;
use api::timesheet::TimesheetDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{
    HiClock, HiDownload, HiPencil, HiPlay, HiPlus, HiRefresh, HiSave, HiStop, HiTag, HiX,
};
use dioxus_free_icons::Icon;

#[component]
pub fn Timesheets() -> Element {
    let timesheets_cache: TimesheetsCache = use_context();
    let projects_cache: ProjectsCache = use_context();
    let activities_cache: ActivitiesCache = use_context();
    let tags_cache: TagsCache = use_context();
    let mut timesheets = use_signal(|| timesheets_cache.read().clone());
    let mut loading = use_signal(|| timesheets_cache.read().is_empty());
    let mut page = use_signal(|| 0_usize);
    let mut running: crate::RunningTimer = use_context();
    let mut projects = use_signal(|| projects_cache.read().clone());
    let mut activities = use_signal(|| activities_cache.read().clone());
    let mut all_tags = use_signal(|| tags_cache.read().clone());
    let mut toasts: Toasts = use_context();
    let user_settings: crate::UserSettings = use_context();
    let workspace_settings: crate::WorkspaceSettings = use_context();

    // Controls which mode the top card shows when no timer is running
    let mut manual_mode = use_signal(|| false);

    // Start timer form
    let mut project_id = use_signal(|| Option::<String>::None);
    let mut activity_id = use_signal(|| Option::<String>::None);
    let mut description = use_signal(String::new);
    let mut billable = use_signal(|| true);

    // Manual entry form
    let mut manual_project_id = use_signal(|| Option::<String>::None);
    let mut manual_activity_id = use_signal(|| Option::<String>::None);
    let mut manual_start = use_signal(String::new);
    let mut manual_end = use_signal(String::new);
    let mut manual_description = use_signal(String::new);
    let mut manual_billable = use_signal(|| true);
    let mut manual_submitting = use_signal(|| false);

    // Edit state for the running timer
    let mut run_project_id = use_signal(|| Option::<String>::None);
    let mut run_activity_id = use_signal(|| Option::<String>::None);
    let mut run_description = use_signal(String::new);
    let mut run_billable = use_signal(|| true);

    // Edit state for recent timesheets
    let mut editing_id = use_signal(|| Option::<String>::None);
    let mut edit_project_id = use_signal(|| Option::<String>::None);
    let mut edit_activity_id = use_signal(|| Option::<String>::None);
    let mut edit_description = use_signal(String::new);
    let mut edit_billable = use_signal(|| true);
    let mut edit_start_time = use_signal(String::new);
    let mut edit_end_time = use_signal(|| Option::<String>::None);

    // Tag panel: which timesheet has the tag panel open
    let mut tagging_id = use_signal(|| Option::<String>::None);
    let mut ts_tags = use_signal(Vec::<TagDto>::new);

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
            if let Some(ref ts) = r {
                run_project_id.set(ts.project_id.clone());
                run_activity_id.set(ts.activity_id.clone());
                run_description.set(ts.description.clone().unwrap_or_default());
                run_billable.set(ts.billable);
            }
            running.set(r);
        }
        if let Ok(list) = api::project::list_projects().await {
            projects.set(list);
        }
        if let Ok(list) = api::activity::list_activities().await {
            activities.set(list);
        }
        if let Ok(list) = api::tag::list_tags().await {
            all_tags.set(list);
        }
        loading.set(false);
    });

    let on_start = move |_| async move {
        let pid = project_id.peek().clone();
        let aid = activity_id.peek().clone();
        let desc = description.peek().clone();
        let bill = *billable.peek();
        let (Some(pid), Some(aid)) = (pid, aid) else {
            return;
        };
        let desc_opt = if desc.is_empty() { None } else { Some(desc) };
        match api::timesheet::start_timesheet(
            Some(pid.clone()),
            Some(aid.clone()),
            desc_opt.clone(),
            bill,
        )
        .await
        {
            Ok(dto) => {
                run_project_id.set(Some(pid));
                run_activity_id.set(Some(aid));
                run_description.set(desc_opt.unwrap_or_default());
                run_billable.set(bill);
                running.set(Some(dto));
                project_id.set(None);
                activity_id.set(None);
                description.set(String::new());
                billable.set(true);
            }
            Err(e) => toasts.push_error(e.to_string()),
        }
    };

    let on_create_manual = move |_| async move {
        let start_local = manual_start.peek().clone();
        let end_local = manual_end.peek().clone();
        let pid = manual_project_id.peek().clone();
        let aid = manual_activity_id.peek().clone();
        let desc_raw = manual_description.peek().clone();
        let bill = *manual_billable.peek();

        if start_local.is_empty() || end_local.is_empty() {
            toasts.push_error("Start and end time are required");
            return;
        }
        manual_submitting.set(true);
        let tz = user_settings.peek().timezone.clone();
        let start = formatting::from_input(&start_local, &tz);
        let end = formatting::from_input(&end_local, &tz);
        let desc = if desc_raw.is_empty() { None } else { Some(desc_raw) };
        match api::timesheet::create_timesheet_manual(pid, aid, start, end, desc, bill).await {
            Ok(dto) => {
                timesheets.write().insert(0, dto);
                manual_project_id.set(None);
                manual_activity_id.set(None);
                manual_start.set(String::new());
                manual_end.set(String::new());
                manual_description.set(String::new());
                manual_billable.set(true);
                toasts.push_success("Timesheet created");
            }
            Err(e) => toasts.push_error(e.to_string()),
        }
        manual_submitting.set(false);
    };

    let on_stop = move |_| async move {
        let maybe_ts = running.peek().clone();
        if let Some(ts) = maybe_ts {
            match api::timesheet::stop_timesheet(ts.id).await {
                Ok(()) => reload().await,
                Err(e) => toasts.push_error(e.to_string()),
            }
        }
    };

    let on_save_running = move |_| async move {
        let ts_id = match running.peek().clone() {
            Some(ts) => ts.id,
            None => return,
        };
        let new_pid = run_project_id.peek().clone();
        let new_aid = run_activity_id.peek().clone();
        let desc = {
            let s = run_description.peek().clone();
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        };
        let bill = *run_billable.peek();

        // Reassign project/activity if changed
        if let (Some(pid), Some(aid)) = (new_pid.clone(), new_aid.clone()) {
            let needs_reassign = running
                .peek()
                .as_ref()
                .map(|ts| {
                    ts.project_id.as_deref() != Some(pid.as_str())
                        || ts.activity_id.as_deref() != Some(aid.as_str())
                })
                .unwrap_or(false);
            if needs_reassign {
                if let Err(e) =
                    api::timesheet::reassign_timesheet(ts_id.clone(), pid.clone(), aid.clone())
                        .await
                {
                    toasts.push_error(e.to_string());
                    return;
                }
            }
        }

        // Update description/billable
        if let Err(e) = api::timesheet::update_timesheet(ts_id.clone(), desc.clone(), bill).await {
            toasts.push_error(e.to_string());
            return;
        }

        // Patch running signal in-place
        if let Some(ts) = running.write().as_mut() {
            if let Some(pid) = new_pid {
                ts.project_id = Some(pid);
            }
            if let Some(aid) = new_aid {
                ts.activity_id = Some(aid);
            }
            ts.description = desc;
            ts.billable = bill;
        }
        toasts.push_success("Timer updated");
    };

    let on_save_edit = move |_| async move {
        let id = match editing_id.peek().clone() {
            Some(id) => id,
            None => return,
        };

        // Update time if the start time field is non-empty.
        // Convert datetime-local values (user timezone) to UTC RFC-3339 before sending.
        let new_start_local = edit_start_time.peek().clone();
        if !new_start_local.is_empty() {
            let tz = user_settings.peek().timezone.clone();
            let new_start = formatting::from_input(&new_start_local, &tz);
            let new_end = edit_end_time
                .peek()
                .as_deref()
                .map(|s| formatting::from_input(s, &tz));
            if let Err(e) =
                api::timesheet::update_timesheet_time(id.clone(), new_start.clone(), new_end.clone())
                    .await
            {
                toasts.push_error(e.to_string());
                return;
            }
            // Patch local state with the UTC values we sent
            if let Some(item) = timesheets.write().iter_mut().find(|x| x.id == id) {
                item.start_time = new_start;
                item.end_time = new_end;
            }
        }

        let new_pid = edit_project_id.peek().clone();
        let new_aid = edit_activity_id.peek().clone();

        // Reassign project/activity if both are selected and either changed
        if let (Some(ref pid), Some(ref aid)) = (&new_pid, &new_aid) {
            let needs_reassign = timesheets
                .read()
                .iter()
                .find(|x| x.id == id)
                .map(|ts| {
                    ts.project_id.as_deref() != Some(pid.as_str())
                        || ts.activity_id.as_deref() != Some(aid.as_str())
                })
                .unwrap_or(false);
            if needs_reassign {
                if let Err(e) =
                    api::timesheet::reassign_timesheet(id.clone(), pid.clone(), aid.clone()).await
                {
                    toasts.push_error(e.to_string());
                    return;
                }
            }
        }

        let desc = {
            let s = edit_description.peek().clone();
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        };
        let bill = *edit_billable.peek();
        if let Err(e) = api::timesheet::update_timesheet(id.clone(), desc.clone(), bill).await {
            toasts.push_error(e.to_string());
            return;
        }
        if let Some(item) = timesheets.write().iter_mut().find(|x| x.id == id) {
            item.description = desc;
            item.billable = bill;
            if let Some(pid) = new_pid {
                item.project_id = Some(pid);
            }
            if let Some(aid) = new_aid {
                item.activity_id = Some(aid);
            }
        }
        editing_id.set(None);
        toasts.push_success("Timesheet updated");
    };

    rsx! {
        DefaultLayout {
            div { class: "space-y-6",

                // ── Timer card ───────────────────────────────────────────────
                match running.read().clone() {
                    Some(ts) => {
                        rsx! {
                            Card { data_size: "md",
                                CardHeader {
                                    CardTitle {
                                        div { class: "flex items-center gap-2",
                                            Icon { icon: HiClock, width: 18, height: 18 }
                                            "Running Timer"
                                            span { class: "text-xs text-secondary font-normal ms-auto",
                                                {
                                                    let s = user_settings.read();
                                                    format!("Started: {}", formatting::format_datetime(&ts.start_time, &s.timezone, &s.date_format))
                                                }
                                            }
                                        }
                                    }
                                }
                                CardContent {
                                    div { class: "grid grid-cols-1 gap-4 md:grid-cols-2",
                                        div { class: "form-field",
                                            label { class: "form-label", "Project" }
                                            Select::<String> {
                                                options: projects.read().iter()
                                                    .map(|p| SelectOption::new(p.id.clone(), p.name.clone()))
                                                    .collect(),
                                                value: run_project_id.read().clone(),
                                                on_change: move |id: String| run_project_id.set(Some(id)),
                                                placeholder: "Select project…".to_string(),
                                            }
                                        }
                                        div { class: "form-field",
                                            label { class: "form-label", "Activity" }
                                            Select::<String> {
                                                options: activities.read().iter()
                                                    .map(|a| SelectOption::new(a.id.clone(), a.name.clone()))
                                                    .collect(),
                                                value: run_activity_id.read().clone(),
                                                on_change: move |id: String| run_activity_id.set(Some(id)),
                                                placeholder: "Select activity…".to_string(),
                                            }
                                        }
                                        div { class: "form-field",
                                            label { class: "form-label", r#for: "run-desc", "Description" }
                                            Input {
                                                id: "run-desc",
                                                placeholder: "What are you working on?",
                                                value: run_description.read().clone(),
                                                oninput: move |e: FormEvent| run_description.set(e.value()),
                                            }
                                        }
                                        div { class: "form-field flex items-center gap-3",
                                            label { class: "form-label", "Billable" }
                                            input {
                                                r#type: "checkbox",
                                                class: "form-checkbox",
                                                checked: *run_billable.read(),
                                                oninput: move |_| { let v = *run_billable.peek(); run_billable.set(!v); },
                                            }
                                        }
                                    }
                                }
                                CardFooter {
                                    Button { onclick: on_save_running,
                                        Icon { icon: HiSave, width: 16, height: 16 }
                                        "Save"
                                    }
                                    Button { onclick: on_stop,
                                        Icon { icon: HiStop, width: 16, height: 16 }
                                        "Stop"
                                    }
                                }
                            }
                        }
                    },
                    None => rsx! {
                        Card { data_size: "md",
                            CardHeader {
                                CardTitle {
                                    div { class: "flex items-center justify-between",
                                        div { class: "flex items-center gap-2",
                                            if *manual_mode.read() {
                                                Icon { icon: HiPlus, width: 18, height: 18 }
                                                "Manual Entry"
                                            } else {
                                                Icon { icon: HiPlay, width: 18, height: 18 }
                                                "Start Timer"
                                            }
                                        }
                                        // Mode toggle
                                        div { class: "flex gap-1 text-xs",
                                            button {
                                                class: if !*manual_mode.read() { "tab-pill tab-pill--active" } else { "tab-pill" },
                                                onclick: move |_| manual_mode.set(false),
                                                "Timer"
                                            }
                                            button {
                                                class: if *manual_mode.read() { "tab-pill tab-pill--active" } else { "tab-pill" },
                                                onclick: move |_| manual_mode.set(true),
                                                "Manual"
                                            }
                                        }
                                    }
                                }
                            }

                            if !*manual_mode.read() {
                                // ── Timer mode ──────────────────────────────
                                CardContent {
                                    div { class: "grid grid-cols-1 gap-4 md:grid-cols-2",
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
                                            label { class: "form-label", r#for: "ts-desc", "Description" }
                                            Input {
                                                id: "ts-desc",
                                                placeholder: "Optional notes…",
                                                value: description.read().clone(),
                                                oninput: move |e: FormEvent| description.set(e.value()),
                                            }
                                        }
                                        div { class: "form-field flex items-center gap-3",
                                            label { class: "form-label", "Billable" }
                                            input {
                                                r#type: "checkbox",
                                                class: "form-checkbox",
                                                checked: *billable.read(),
                                                oninput: move |_| { let v = *billable.peek(); billable.set(!v); },
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
                            } else {
                                // ── Manual entry mode ────────────────────────
                                CardContent {
                                    div { class: "grid grid-cols-1 gap-4 md:grid-cols-2",
                                        div { class: "form-field",
                                            label { class: "form-label", r#for: "m-start", "Start time" }
                                            input {
                                                id: "m-start",
                                                r#type: "datetime-local",
                                                class: "input",
                                                value: manual_start.read().clone(),
                                                oninput: move |e: FormEvent| manual_start.set(e.value()),
                                            }
                                        }
                                        div { class: "form-field",
                                            label { class: "form-label", r#for: "m-end", "End time" }
                                            input {
                                                id: "m-end",
                                                r#type: "datetime-local",
                                                class: "input",
                                                value: manual_end.read().clone(),
                                                oninput: move |e: FormEvent| manual_end.set(e.value()),
                                            }
                                        }
                                        div { class: "form-field",
                                            label { class: "form-label", "Project" }
                                            Select::<String> {
                                                options: projects.read().iter()
                                                    .map(|p| SelectOption::new(p.id.clone(), p.name.clone()))
                                                    .collect(),
                                                value: manual_project_id.read().clone(),
                                                on_change: move |id: String| manual_project_id.set(Some(id)),
                                                placeholder: "Select project…".to_string(),
                                            }
                                        }
                                        div { class: "form-field",
                                            label { class: "form-label", "Activity" }
                                            Select::<String> {
                                                options: activities.read().iter()
                                                    .map(|a| SelectOption::new(a.id.clone(), a.name.clone()))
                                                    .collect(),
                                                value: manual_activity_id.read().clone(),
                                                on_change: move |id: String| manual_activity_id.set(Some(id)),
                                                placeholder: "Select activity…".to_string(),
                                            }
                                        }
                                        div { class: "form-field md:col-span-2",
                                            label { class: "form-label", r#for: "m-desc", "Description" }
                                            Input {
                                                id: "m-desc",
                                                placeholder: "Optional notes…",
                                                value: manual_description.read().clone(),
                                                oninput: move |e: FormEvent| manual_description.set(e.value()),
                                            }
                                        }
                                        div { class: "form-field flex items-center gap-3",
                                            label { class: "form-label", "Billable" }
                                            input {
                                                r#type: "checkbox",
                                                class: "form-checkbox",
                                                checked: *manual_billable.read(),
                                                oninput: move |_| { let v = *manual_billable.peek(); manual_billable.set(!v); },
                                            }
                                        }
                                    }
                                }
                                CardFooter {
                                    Button {
                                        onclick: on_create_manual,
                                        disabled: *manual_submitting.read(),
                                        if *manual_submitting.read() {
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
                    },
                }

                // ── Recent timesheets ────────────────────────────────────────
                {
                    let total = timesheets.read().len();
                    let current_page = *page.read();
                    let page_size = 20_usize;
                    let page_items: Vec<TimesheetDto> = timesheets
                        .read()
                        .iter()
                        .skip(current_page * page_size)
                        .take(page_size)
                        .cloned()
                        .collect();
                    let ts_columns = vec![
                        ColumnDef::new("Project / Activity"),
                        ColumnDef::new("Start").width("160px"),
                        ColumnDef::new("Duration").right().width("90px"),
                        ColumnDef::new("Flags").width("100px"),
                        ColumnDef::new("").width("100px"),
                    ];
                    let col_count = ts_columns.len();

                    rsx! {
                div { class: "island",
                    div { class: "island-header",
                        span { class: "island-title", "Recent" }
                    }
                    DataTable {
                        columns: ts_columns,
                        total,
                        page: current_page,
                        page_size,
                        loading: *loading.read(),
                        on_page_change: move |p| page.set(p),

                    for ts in page_items {
                        {
                            let t = ts.clone();
                            let tsid = t.id.clone();
                            let tsid2 = t.id.clone();
                            let is_editing = editing_id.read().as_deref() == Some(t.id.as_str());
                            let is_tagging = tagging_id.read().as_deref() == Some(t.id.as_str());
                            let proj_name = t.project_id.as_ref()
                                .and_then(|pid| projects.read().iter().find(|p| &p.id == pid).map(|p| p.name.clone()))
                                .unwrap_or_else(|| "—".to_string());
                            let act_name = t.activity_id.as_ref()
                                .and_then(|aid| activities.read().iter().find(|a| &a.id == aid).map(|a| a.name.clone()))
                                .unwrap_or_else(|| "—".to_string());
                            let duration_str = t.duration.map(|dur| {
                                let h = dur / 3600;
                                let m = (dur % 3600) / 60;
                                if h > 0 { format!("{h}h {m}m") } else { format!("{m}m") }
                            });
                            let date_str = {
                                let s = user_settings.read();
                                formatting::format_datetime(&t.start_time, &s.timezone, &s.date_format)
                            };

                            rsx! {
                                TableRow { key: "{t.id}", muted: t.exported,
                                    TableCell {
                                        div { class: "flex flex-col gap-0.5",
                                            span { class: "font-medium text-sm", "{proj_name}" }
                                            span { class: "text-xs text-secondary", "{act_name}" }
                                            if let Some(ref desc) = t.description {
                                                span { class: "text-xs text-secondary italic", "{desc}" }
                                            }
                                        }
                                    }
                                    TableCell { mono: true, "{date_str}" }
                                    TableCell { align: crate::components::atoms::ColAlign::Right, mono: true,
                                        if let Some(ref d) = duration_str {
                                            span { "{d}" }
                                        } else {
                                            span { class: "text-secondary", "—" }
                                        }
                                    }
                                    TableCell {
                                        div { class: "flex flex-col gap-0.5 text-xs",
                                            if t.billable {
                                                span { class: "text-success", "Billable" }
                                            }
                                            if let Some(r) = t.rate {
                                                span { class: "font-medium",
                                                    { formatting::format_money(r, &workspace_settings.read().currency) }
                                                }
                                            }
                                            if t.exported {
                                                span { class: "text-secondary", "Exported" }
                                            }
                                        }
                                    }
                                    TableCell {
                                        div { class: "flex gap-1",
                                            if is_editing || is_tagging {
                                                Button {
                                                    onclick: move |_| { editing_id.set(None); tagging_id.set(None); },
                                                    Icon { icon: HiX, width: 14, height: 14 }
                                                }
                                            } else {
                                                Button {
                                                    onclick: move |_| {
                                                        let edit_ts = timesheets.read()
                                                            .iter()
                                                            .find(|x| x.id == tsid)
                                                            .cloned();
                                                        if let Some(et) = edit_ts {
                                                            edit_project_id.set(et.project_id.clone());
                                                            edit_activity_id.set(et.activity_id.clone());
                                                            edit_description.set(et.description.clone().unwrap_or_default());
                                                            edit_billable.set(et.billable);
                                                            let tz = user_settings.peek().timezone.clone();
                                                            edit_start_time.set(formatting::to_input(&et.start_time, &tz));
                                                            edit_end_time.set(et.end_time.as_deref().map(|s| formatting::to_input(s, &tz)));
                                                            editing_id.set(Some(et.id));
                                                            tagging_id.set(None);
                                                        }
                                                    },
                                                    Icon { icon: HiPencil, width: 14, height: 14 }
                                                }
                                                Button {
                                                    onclick: move |_| {
                                                        let tsid2 = tsid2.clone();
                                                        async move {
                                                            if tagging_id.peek().as_deref() == Some(tsid2.as_str()) {
                                                                tagging_id.set(None);
                                                            } else {
                                                                match api::tag::list_timesheet_tags(tsid2.clone()).await {
                                                                    Ok(tags) => ts_tags.set(tags),
                                                                    Err(e) => toasts.push_error(e.to_string()),
                                                                }
                                                                tagging_id.set(Some(tsid2.clone()));
                                                                editing_id.set(None);
                                                            }
                                                        }
                                                    },
                                                    Icon { icon: HiTag, width: 14, height: 14 }
                                                }
                                                if !t.exported && t.end_time.is_some() {
                                                    {
                                                        let tsid_ex = t.id.clone();
                                                        rsx! {
                                                            Button {
                                                                onclick: move |_| {
                                                                    let tsid_ex = tsid_ex.clone();
                                                                    async move {
                                                                        match api::timesheet::export_timesheet(tsid_ex.clone()).await {
                                                                            Ok(()) => {
                                                                                // Inline patch — instant UI feedback
                                                                                if let Some(item) = timesheets.write().iter_mut().find(|x| x.id == tsid_ex) {
                                                                                    item.exported = true;
                                                                                }
                                                                                toasts.push_success("Marked as exported");
                                                                            }
                                                                            Err(e) => toasts.push_error(e.to_string()),
                                                                        }
                                                                    }
                                                                },
                                                                Icon { icon: HiDownload, width: 14, height: 14 }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } // div flex gap-1
                                    } // TableCell actions
                                } // TableRow

                                    // ── Inline edit form ──────────────────────────────────
                                    if is_editing {
                                        TableExpandRow { col_count,
                                            div { class: "grid grid-cols-1 gap-4 md:grid-cols-2",
                                                div { class: "form-field",
                                                    label { class: "form-label", r#for: "et-start", "Start time" }
                                                    input {
                                                        id: "et-start",
                                                        r#type: "datetime-local",
                                                        class: "input",
                                                        value: edit_start_time.read().clone(),
                                                        oninput: move |e: FormEvent| edit_start_time.set(e.value()),
                                                    }
                                                }
                                                if edit_end_time.read().is_some() {
                                                    div { class: "form-field",
                                                        label { class: "form-label", r#for: "et-end", "End time" }
                                                        input {
                                                            id: "et-end",
                                                            r#type: "datetime-local",
                                                            class: "input",
                                                            value: edit_end_time.read().clone().unwrap_or_default(),
                                                            oninput: move |e: FormEvent| {
                                                                let v = e.value();
                                                                edit_end_time.set(if v.is_empty() { None } else { Some(v) });
                                                            },
                                                        }
                                                    }
                                                }
                                                div { class: "form-field",
                                                    label { class: "form-label", "Project" }
                                                    Select::<String> {
                                                        options: projects.read().iter()
                                                            .map(|p| SelectOption::new(p.id.clone(), p.name.clone()))
                                                            .collect(),
                                                        value: edit_project_id.read().clone(),
                                                        on_change: move |id: String| edit_project_id.set(Some(id)),
                                                        placeholder: "Select project…".to_string(),
                                                    }
                                                }
                                                div { class: "form-field",
                                                    label { class: "form-label", "Activity" }
                                                    Select::<String> {
                                                        options: activities.read().iter()
                                                            .map(|a| SelectOption::new(a.id.clone(), a.name.clone()))
                                                            .collect(),
                                                        value: edit_activity_id.read().clone(),
                                                        on_change: move |id: String| edit_activity_id.set(Some(id)),
                                                        placeholder: "Select activity…".to_string(),
                                                    }
                                                }
                                                div { class: "form-field md:col-span-2",
                                                    label { class: "form-label", r#for: "et-desc", "Description" }
                                                    Input {
                                                        id: "et-desc",
                                                        placeholder: "Optional notes…",
                                                        value: edit_description.read().clone(),
                                                        oninput: move |e: FormEvent| edit_description.set(e.value()),
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
                                            div { class: "flex gap-2 mt-2",
                                                Button { onclick: on_save_edit,
                                                    Icon { icon: HiSave, width: 14, height: 14 }
                                                    "Save"
                                                }
                                                Button {
                                                    onclick: move |_| editing_id.set(None),
                                                    Icon { icon: HiX, width: 14, height: 14 }
                                                    "Cancel"
                                                }
                                            }
                                        }
                                    }

                                    // ── Tag panel ──────────────────────────────────────────
                                    if is_tagging {
                                        {
                                            let tsid_tag = t.id.clone();
                                            rsx! {
                                                TableExpandRow { col_count,
                                                    p { class: "text-xs font-medium text-secondary mb-2", "Tags" }
                                                    div { class: "flex flex-wrap gap-2",
                                                        for tag in all_tags.read().clone() {
                                                            {
                                                                let tag_id = tag.id.clone();
                                                                let tag_id2 = tag.id.clone();
                                                                let tsid_a = tsid_tag.clone();
                                                                let tsid_b = tsid_tag.clone();
                                                                let is_applied = ts_tags.read().iter().any(|t| t.id == tag.id);
                                                                rsx! {
                                                                    button {
                                                                        key: "{tag.id}",
                                                                        class: if is_applied { "tag-pill tag-pill--active" } else { "tag-pill" },
                                                                        onclick: move |_| {
                                                                            let tag_id = tag_id.clone();
                                                                            let tsid_a = tsid_a.clone();
                                                                            let tsid_b = tsid_b.clone();
                                                                            let _ = tag_id2.clone();
                                                                            async move {
                                                                                let result = if is_applied {
                                                                                    api::tag::untag_timesheet(tag_id.clone(), tsid_a.clone()).await
                                                                                } else {
                                                                                    api::tag::tag_timesheet(tag_id.clone(), tsid_a.clone()).await
                                                                                };
                                                                                if result.is_ok() {
                                                                                    if let Ok(tags) = api::tag::list_timesheet_tags(tsid_b.clone()).await {
                                                                                        ts_tags.set(tags);
                                                                                    }
                                                                                }
                                                                            }
                                                                        },
                                                                        "{tag.name}"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } // rsx! row
                            } // inner { let t = ... } block
                        } // for ts in page_items
                    } // DataTable
                } // island div
                    } // rsx! island wrapper
                } // outer { let ts_columns = ... } block
            } // space-y-6 div
        } // DefaultLayout
    } // outer rsx!
} // fn Timesheets
