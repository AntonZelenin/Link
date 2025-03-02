use crate::{AuthError, AuthResponse};
use lcore::api::constants::AUTH_SERVICE_API_URL;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::{use_resource, use_signal};
use dioxus::prelude::*;

#[component]
pub fn LoginModal(is_authenticated: Signal<bool>, show_modal: Signal<bool>) -> Element {
    let mut active_tab = use_signal(|| "login".to_string());
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| String::new());
    let mut processing = use_signal(|| false);

    rsx! {
        div {
            class: "login-modal",
            div {
                class: "login-modal-content",
                div {
                    class: "login-modal-header",
                    "Welcome to ",
                    span {
                        class: "highlighted-text",
                        "<Lїnk>"
                    }
                }
                div {
                    class: "login-modal-tabs",
                    div {
                        class: format_args!("login-modal-tab {}", if *active_tab.read() == "login" { "active" } else { "" }),
                        onclick: move |_| active_tab.set("login".to_string()),
                        "Login"
                    }
                    div {
                        class: format_args!("login-modal-tab {}", if *active_tab.read() == "register" { "active" } else { "" }),
                        onclick: move |_| active_tab.set("register".to_string()),
                        "Register"
                    }
                }
                form {
                    onsubmit:
                        move |ev| {
                            ev.prevent_default();
                            let auth_result = use_resource(move || async move {
                                perform_auth(
                                    active_tab.read().clone(),
                                    username.read().clone(),
                                    password.read().clone(),
                                ).await
                            });

                            match auth_result.read().as_ref() {
                                Some(Ok(auth_data)) => {
                                    if let Some(storage) = web_sys::window()
                                        .and_then(|w| w.local_storage().ok())
                                        .flatten()
                                    {
                                        storage.set_item("access_token", &auth_data.access_token).ok();
                                        storage.set_item("refresh_token", &auth_data.refresh_token).ok();
                                        storage.set_item("user_id", &auth_data.user_id).ok();
                                    }
                                    is_authenticated.set(true);
                                    show_modal.set(false);
                                }
                                Some(Err(e)) => error.set(e.clone()),
                                None => {}
                            }
                            processing.set(false);
                    },
                    class: "login-modal-form",

                    input {
                        class: "login-modal-input",
                        "type": "text",
                        placeholder: "Username",
                        value: "{username}",
                        oninput: move |e| username.set(e.value().clone())
                    }

                    input {
                        class: "login-modal-input",
                        "type": "password",
                        placeholder: "Password",
                        value: "{password}",
                        oninput: move |e| password.set(e.value().clone())
                    }

                    if !error.read().is_empty() {
                        div {
                            class: "login-modal-error",
                            "{error}"
                        }
                    }

                    div {
                        class: "login-modal-buttons",
                        button {
                            "type": "button",
                            class: "login-modal-button cancel",
                            onclick: move |_| show_modal.set(false),
                            "Cancel"
                        }
                        button {
                            "type": "submit",
                            class: "login-modal-button submit",
                            disabled: *processing.read(),
                            {if *processing.read() {
                                "Processing..."
                            } else if *active_tab.read() == "login" {
                                "Login"
                            } else {
                                "Register"
                            }}
                        }
                    }
                }
            }
        }
    }
}

async fn perform_auth(
    active_tab: String,
    username: String,
    password: String,
) -> Result<AuthResponse, String> {
    if username.is_empty() || password.is_empty() {
        return Err("Please fill in all fields".into());
    }
    let endpoint = if active_tab == "login" {
        "login"
    } else {
        "users"
    };
    let url = AUTH_SERVICE_API_URL.to_string() + &endpoint;

    let resp = reqwest::Client::new()
        .post(&url)
        .json(&serde_json::json!({ "username": username, "password": password }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        resp.json::<AuthResponse>()
            .await
            .map_err(|_| "Failed to parse auth response".into())
    } else {
        let err = resp.json::<AuthError>().await.unwrap_or(AuthError {
            detail: "Unknown error".into(),
        });
        Err(err.detail)
    }
}
