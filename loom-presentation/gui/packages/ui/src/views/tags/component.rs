use crate::components::atoms::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::components::atoms::{Button, Input, ToastMessage, Toasts};
use crate::layouts::DefaultLayout;
use api::tag::TagDto;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{HiPencil, HiPlus, HiSave, HiTag, HiX};
use dioxus_free_icons::Icon;

#[component]
pub fn Tags() -> Element {
    let mut tags = use_signal(Vec::<TagDto>::new);
    let mut toasts: Toasts = use_context();

    let mut new_name = use_signal(String::new);

    let mut editing_id = use_signal(|| Option::<String>::None);
    let mut edit_name = use_signal(String::new);

    use_resource(move || async move {
        match api::tag::list_tags().await {
            Ok(list) => tags.set(list),
            Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
        }
    });

    let on_create = move |_| async move {
        let name = new_name.peek().clone();
        if name.is_empty() {
            return;
        }
        match api::tag::create_tag(name).await {
            Ok(dto) => {
                tags.write().push(dto);
                new_name.set(String::new());
                toasts.write().push(ToastMessage::success("Tag created"));
            }
            Err(e) => toasts.write().push(ToastMessage::error(e.to_string())),
        }
    };

    let on_save = move |_| async move {
        let id = match editing_id.peek().clone() {
            Some(id) => id,
            None => return,
        };
        let name = edit_name.peek().clone();
        if name.is_empty() {
            return;
        }
        if let Err(e) = api::tag::rename_tag(id.clone(), name.clone()).await {
            toasts.write().push(ToastMessage::error(e.to_string()));
            return;
        }
        // Update local list
        if let Some(tag) = tags.write().iter_mut().find(|t| t.id == id) {
            tag.name = name;
        }
        editing_id.set(None);
        toasts.write().push(ToastMessage::success("Tag renamed"));
    };

    rsx! {
        DefaultLayout {
            div { class: "space-y-6",

                // ── Create form ──────────────────────────────────────────────
                Card { data_size: "md",
                    CardHeader {
                        CardTitle {
                            div { class: "flex items-center gap-2",
                                Icon { icon: HiTag, width: 18, height: 18 }
                                "New Tag"
                            }
                        }
                    }
                    CardContent {
                        div { class: "form-field",
                            label { class: "form-label", r#for: "tag-name", "Name" }
                            Input {
                                id: "tag-name",
                                placeholder: "e.g. urgent",
                                value: new_name.read().clone(),
                                oninput: move |e: FormEvent| new_name.set(e.value()),
                            }
                        }
                    }
                    CardFooter {
                        Button { onclick: on_create,
                            Icon { icon: HiPlus, width: 16, height: 16 }
                            "Create"
                        }
                    }
                }

                // ── Tag list ─────────────────────────────────────────────────
                div { class: "flex flex-col gap-3",
                    for tag in tags.read().clone() {
                        {
                            let t = tag.clone();
                            let tid = t.id.clone();
                            let is_editing = editing_id.read().as_deref() == Some(t.id.as_str());

                            rsx! {
                                Card { key: "{t.id}",
                                    CardContent {
                                        if is_editing {
                                            div { class: "flex items-center gap-3",
                                                div { class: "form-field flex-1",
                                                    label { class: "form-label", r#for: "et-name", "Name" }
                                                    Input {
                                                        id: "et-name",
                                                        value: edit_name.read().clone(),
                                                        oninput: move |e: FormEvent| edit_name.set(e.value()),
                                                    }
                                                }
                                                div { class: "flex gap-2 mt-5",
                                                    Button { onclick: on_save,
                                                        Icon { icon: HiSave, width: 15, height: 15 }
                                                        "Save"
                                                    }
                                                    Button {
                                                        onclick: move |_| editing_id.set(None),
                                                        Icon { icon: HiX, width: 15, height: 15 }
                                                        "Cancel"
                                                    }
                                                }
                                            }
                                        } else {
                                            div { class: "flex items-center justify-between",
                                                span { class: "font-medium", "{t.name}" }
                                                Button {
                                                    onclick: move |_| {
                                                        let tag_name = tags.read()
                                                            .iter()
                                                            .find(|x| x.id == tid)
                                                            .map(|x| x.name.clone())
                                                            .unwrap_or_default();
                                                        edit_name.set(tag_name);
                                                        editing_id.set(Some(tid.clone()));
                                                    },
                                                    Icon { icon: HiPencil, width: 15, height: 15 }
                                                    "Rename"
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
