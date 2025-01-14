use crate::{AuthError, AuthResponse};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::{use_resource, use_signal};
use dioxus::prelude::*;

#[component]
pub fn LoginModal(is_authenticated: Signal<bool>, show_modal: Signal<bool>) -> Element {
    let mut active_tab = use_signal(|| "login".to_string());
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(String::new);

    let mut auth_request = use_resource(move || {
        let username = username.read().clone();
        let password = password.read().clone();
        let is_login = *active_tab.read() == "login";

        async move {
            if username.is_empty() || password.is_empty() {
                return Err("Please fill in all fields".to_string());
            }

            let endpoint = if is_login { "login" } else { "users" };
            let url = format!("http://185.191.177.247:55800/api/auth/v1/{}", endpoint);

            let response = reqwest::Client::new()
                .post(&url)
                .json(&serde_json::json!({
                    "username": username,
                    "password": password,
                }))
                .send()
                .await
                .map_err(|e| e.to_string())?;

            if response.status().is_success() {
                let auth_data: AuthResponse = response.json().await.map_err(|e| e.to_string())?;

                if let Some(storage) = web_sys::window()
                    .and_then(|w| w.local_storage().ok())
                    .and_then(|s| s)
                {
                    storage
                        .set_item("access_token", &auth_data.access_token)
                        .ok();
                    storage
                        .set_item("refresh_token", &auth_data.refresh_token)
                        .ok();
                    storage.set_item("user_id", &auth_data.user_id).ok();
                }

                Ok(())
            } else {
                let error: AuthError = response.json().await.map_err(|e| e.to_string())?;
                Err(error.detail)
            }
        }
    });

    // let auth_value = auth_request.value().read();
    let auth_value = auth_request.cloned();

    // Update error state based on auth request result
    if let Some(Err(err)) = auth_value.as_ref() {
        error.set(err.clone());
    }

    // Check for successful authentication
    if let Some(Ok(_)) = auth_value.as_ref() {
        is_authenticated.set(true);
        show_modal.set(false);
    }

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
                        class: format_args!(
                            "login-modal-tab {}",
                            if *active_tab.read() == "login" { "active" } else { "" }
                        ),
                        onclick: move |_| active_tab.set("login".to_string()),
                        "Login"
                    }
                    div {
                        class: format_args!(
                            "login-modal-tab {}",
                            if *active_tab.read() == "register" { "active" } else { "" }
                        ),
                        onclick: move |_| active_tab.set("register".to_string()),
                        "Register"
                    }
                }

                form {
                    onsubmit: move |ev| {
                        ev.prevent_default();
                        auth_request.restart();
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

                    {
                        if !error.read().is_empty() {
                            rsx! {
                                div {
                                    class: "login-modal-error",
                                    "{error}"
                                }
                            }
                        } else {
                            rsx! {}
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
                            disabled: auth_value.is_some(),
                            {if auth_value.is_some() {
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
