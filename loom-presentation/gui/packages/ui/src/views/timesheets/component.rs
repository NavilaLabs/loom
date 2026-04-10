use crate::components::atoms::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::components::atoms::{
    Button, Input, Select, SelectOption, SkeletonListItem, ToastMessage, Toasts,
};
use crate::layouts::DefaultLayout;
use api::activity::ActivityDto;
use api::project::ProjectDto;
use api::tag::TagDto;
use api::timesheet::TimesheetDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{
    HiClock, HiDownload, HiPencil, HiPlay, HiSave, HiStop, HiTag, HiX,
};
use dioxus_free_icons::Icon;

#[component]
pub fn Timesheets() -> Element {
    let mut timesheets = use_signal(Vec::<TimesheetDto>::new);
    let mut loading = use_signal(|| true);
    let mut running: crate::RunningTimer = use_context();
    let mut projects = use_signal(Vec::<ProjectDto>::new);
    let mut activities = use_signal(Vec::<ActivityDto>::new);
    let mut all_tags = use_signal(Vec::<TagDto>::new);
    let mut toasts: Toasts = use_context();

    // Start form
    let mut project_id = use_signal(|| Option::<String>::None);
    let mut activity_id = use_signal(|| Option::<String>::None);
    let mut description = use_signal(String::new);
    let mut billable = use_signal(|| true);

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
                    toasts.write().push(ToastMessage::error(e.to_string()));
                    return;
                }
            }
        }

        // Update description/billable
        if let Err(e) = api::timesheet::update_timesheet(ts_id.clone(), desc.clone(), bill).await {
            toasts.write().push(ToastMessage::error(e.to_string()));
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
        toasts.write().push(ToastMessage::success("Timer updated"));
    };

    let on_save_edit = move |_| async move {
        let id = match editing_id.peek().clone() {
            Some(id) => id,
            None => return,
        };
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
                    toasts.write().push(ToastMessage::error(e.to_string()));
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
            toasts.write().push(ToastMessage::error(e.to_string()));
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
        toasts
            .write()
            .push(ToastMessage::success("Timesheet updated"));
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
                                                "Started: {ts.start_time}"
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
                                    div { class: "flex items-center gap-2",
                                        Icon { icon: HiPlay, width: 18, height: 18 }
                                        "Start Timer"
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
                        }
                    },
                }

                // ── Recent timesheets ────────────────────────────────────────
                h2 { class: "text-base font-semibold", "Recent" }
                div { class: "flex flex-col gap-3",
                    if *loading.read() {
                        for _ in 0..5 {
                            SkeletonListItem {}
                        }
                    }
                    for ts in timesheets.read().clone() {
                        {
                            let t = ts.clone();
                            let tsid = t.id.clone();
                            let tsid2 = t.id.clone();
                            let is_editing = editing_id.read().as_deref() == Some(t.id.as_str());
                            let is_tagging = tagging_id.read().as_deref() == Some(t.id.as_str());
                            let proj_name = t.project_id.as_ref()
                                .and_then(|pid| projects.read().iter().find(|p| &p.id == pid).map(|p| p.name.clone()))
                                .unwrap_or_else(|| "Unassigned".to_string());
                            let act_name = t.activity_id.as_ref()
                                .and_then(|aid| activities.read().iter().find(|a| &a.id == aid).map(|a| a.name.clone()))
                                .unwrap_or_else(|| "Unassigned".to_string());

                            rsx! {
                                Card { key: "{t.id}",
                                    CardContent {
                                        div { class: "flex flex-col gap-3",

                                            // Row: name + actions
                                            div { class: "flex items-start justify-between",
                                                div { class: "flex flex-col gap-1",
                                                    span { class: "font-medium", "{proj_name} / {act_name}" }
                                                    p { class: "text-xs text-secondary", "{t.start_time}" }
                                                    if let Some(dur) = t.duration {
                                                        p { class: "text-xs text-secondary",
                                                            {
                                                                let h = dur / 3600;
                                                                let m = (dur % 3600) / 60;
                                                                if h > 0 { format!("{h}h {m}m") } else { format!("{m}m") }
                                                            }
                                                        }
                                                    }
                                                    if t.billable {
                                                        span { class: "text-xs text-success", "Billable" }
                                                    }
                                                    if let Some(r) = t.rate {
                                                        span { class: "text-xs font-medium",
                                                            {format!("{:.2} EUR", r as f64 / 100.0)}
                                                        }
                                                    }
                                                    if t.exported {
                                                        span { class: "text-xs text-secondary", "Exported" }
                                                    }
                                                }
                                                div { class: "flex gap-2",
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
                                                                        Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
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
                                                                                    if let Ok(list) = api::timesheet::list_timesheets().await {
                                                                                        timesheets.set(list);
                                                                                    }
                                                                                    toasts.write().push(ToastMessage::success("Marked as exported"));
                                                                                }
                                                                                Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
                                                                            }
                                                                        }
                                                                    },
                                                                    Icon { icon: HiDownload, width: 14, height: 14 }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            // Inline edit form
                                            if is_editing {
                                                div { class: "border-t border-surface-tonal pt-3 flex flex-col gap-3",
                                                    div { class: "grid grid-cols-1 gap-4 md:grid-cols-2",
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
                                                    }
                                                    div { class: "form-field",
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
                                                    div { class: "flex gap-2",
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

                                            // Tag panel
                                            if is_tagging {
                                                {
                                                    let tsid_tag = t.id.clone();
                                                    rsx! {
                                                        div { class: "border-t border-surface-tonal pt-3 flex flex-col gap-2",
                                                            p { class: "text-xs font-medium text-secondary", "Tags" }
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
