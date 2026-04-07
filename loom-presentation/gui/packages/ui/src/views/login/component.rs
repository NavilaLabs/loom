use crate::components::atoms::{Button, Card, CardContent, CardFooter, Form, FormField, Input, Label};
use crate::layouts::DefaultLayout;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let mut submitting = use_signal(|| false);

    let navigator = use_navigator();

    let on_submit = move |_| {
        let email = email.read().clone();
        let password = password.read().clone();

        async move {
            submitting.set(true);
            error.set(None);

            match api::login::login(email, password).await {
                Ok(_token) => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        let js = format!(
                            "localStorage.setItem('auth_token', '{}');",
                            _token.replace('\'', "\\'")
                        );
                        let _ = document::eval(&js).await;
                    }
                    navigator.push("/dashboard");
                }
                Err(e) => {
                    error.set(Some(e.to_string()));
                    submitting.set(false);
                }
            }
        }
    };

    rsx! {
        DefaultLayout {
            Card {
                class: "w-full",
                data_size: "md",
                CardContent {
                    Form {
                        FormField {
                            Label { html_for: "email", class: "w-full", "Email" }
                            Input {
                                id: "email",
                                r#type: "email",
                                class: "w-full",
                                oninput: move |e: FormEvent| email.set(e.value()),
                            }
                        }
                        FormField {
                            Label { html_for: "password", class: "w-full", "Password" }
                            Input {
                                id: "password",
                                r#type: "password",
                                class: "w-full",
                                oninput: move |e: FormEvent| password.set(e.value()),
                            }
                        }
                        if let Some(msg) = error.read().as_deref() {
                            p { class: "text-red-500 text-sm mt-2", "{msg}" }
                        }
                    }
                }
                CardFooter {
                    Button {
                        class: "ms-auto",
                        r#type: "submit",
                        disabled: *submitting.read(),
                        onclick: on_submit,
                        if *submitting.read() { "Signing in…" } else { "Sign in" }
                    }
                }
            }
        }
    }
}
