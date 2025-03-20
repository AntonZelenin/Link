use crate::{storage, SharedClient};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_signal;
use dioxus::prelude::*;
use lcore::api::schemas::{AuthError, LoginRequest, RegisterError, RegisterRequest};
use lcore::third_party::utils::form_values_to_string;
use lcore::{auth, utils};
use log::info;
use validator::Validate;

#[component]
pub fn LoginModal(is_authenticated: Signal<bool>, show_modal: Signal<bool>) -> Element {
    let mut active_tab = use_signal(|| "login".to_string());

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

                if *active_tab.read() == "login" {
                    LoginForm { is_authenticated, show_modal }
                }
                if *active_tab.read() == "register" {
                    RegisterForm { is_authenticated, show_modal }
                }
            }
        }
    }
}

#[component]
pub fn LoginForm(is_authenticated: Signal<bool>, show_modal: Signal<bool>) -> Element {
    let mut error = use_signal(|| String::new());
    let mut processing = use_signal(|| false);
    let client = use_context::<SharedClient>();
    info!("Test log message");

    rsx! {
        form {
            onsubmit: move |ev| {
                processing.set(true);
                error.set(String::new());

                let req = match utils::from_map::<LoginRequest>(
                    &form_values_to_string(&ev.values())
                ) {
                    Ok(request) => request,
                    Err(_) => {
                        error.set("Invalid form data".to_string());
                        processing.set(false);
                        return;
                    }
                };

                // client is an Arc, so when we clone it, we're just cloning the reference
                // cloning  is needed to move the client into the async block
                let mut client = client.clone();

                spawn(async move {
                    let storage = storage::get_storage();
                    match auth::login(req, &mut client, &storage).await {
                        Ok(()) => {
                            is_authenticated.set(true);
                            show_modal.set(false);
                        }
                        Err(e) => {
                            match e {
                                AuthError::ApiError(msg) => {
                                    error.set(msg);
                                }
                            };
                        }
                    }
                    processing.set(false);
                });
            },

            class: "login-modal-form",

            div {
                class: "field-container",
                input {
                    class: "login-modal-input",
                    r#type: "text",
                    placeholder: "Username",
                    name: "username"
                }
            }

            div {
                class: "field-container",
                input {
                    class: "login-modal-input",
                    r#type: "password",
                    placeholder: "Password",
                    name: "password"
                }

            }

            div {
                class: "login-modal-error",
                "{error}"
            }

            div {
                class: "login-modal-buttons",
                button {
                    r#type: "button",
                    class: "login-modal-button cancel",
                    onclick: move |_| show_modal.set(false),
                    "Cancel"
                }
                button {
                    r#type: "submit",
                    class: "login-modal-button submit",
                    disabled: *processing.read(),
                    { if *processing.read() {
                        "Processing..."
                    } else {
                        "Login"
                    }}
                }
            }
        }
    }
}

#[component]
pub fn RegisterForm(is_authenticated: Signal<bool>, show_modal: Signal<bool>) -> Element {
    let mut error_username = use_signal(|| String::new());
    let mut error_password = use_signal(|| String::new());
    let mut processing = use_signal(|| false);
    let client = use_context::<SharedClient>();

    rsx! {
        form {
            onsubmit: move |ev| {
                processing.set(true);
                error_username.set("".to_string());
                error_password.set("".to_string());

                let req_result = utils::from_map::<RegisterRequest>(&form_values_to_string(&ev.values()));
                let req = match req_result {
                    Ok(req) => req,
                    Err(_) => {
                        error_username.set("Invalid form data".to_string());
                        processing.set(false);
                        return;
                    }
                };

                if let Err(validation_errors) = req.validate() {
                    if let Some(errs) = validation_errors.field_errors().get("username") {
                        if let Some(e) = errs.first() {
                            if let Some(m) = &e.message {
                                error_username.set(m.to_string());
                            }
                        }
                    }
                    if let Some(errs) = validation_errors.field_errors().get("password") {
                        if let Some(e) = errs.first() {
                            if let Some(m) = &e.message {
                                error_password.set(m.to_string());
                            }
                        }
                    }
                    processing.set(false);
                    return;
                }

                let client = client.clone();
                spawn(async move {
                    match client.register(req).await {
                        Ok(auth_data) => {
                            if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok()).flatten() {
                                storage.set_item("access_token", &auth_data.access_token).ok();
                                storage.set_item("refresh_token", &auth_data.refresh_token).ok();
                                storage.set_item("user_id", &auth_data.user_id).ok();
                            }
                            is_authenticated.set(true);
                            show_modal.set(false);
                        }
                        Err(e) => {
                            match e {
                                RegisterError::ValidationErrors(map) => {
                                    if let Some(username_err) = map.get("username") {
                                        error_username.set(username_err.clone());
                                    }
                                    if let Some(password_err) = map.get("password") {
                                        error_password.set(password_err.clone());
                                    }
                                }
                                RegisterError::ApiError(msg) => {
                                    error_username.set(msg);
                                }
                            }
                        }
                    }
                    processing.set(false);
                });
            },
            class: "login-modal-form",

            div {
                class: "field-container",
                input {
                    class: "login-modal-input",
                    r#type: "text",
                    placeholder: "Username",
                    name: "username"
                }
                div {
                    class: "login-modal-error",
                    "{error_username}"
                }
            }
            div {
                class: "field-container",
                input {
                    class: "login-modal-input",
                    r#type: "password",
                    placeholder: "Password",
                    name: "password"
                }
                div {
                    class: "login-modal-error",
                    "{error_password}"
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
                    } else {
                        "Register"
                    }}
                }
            }
        }
    }
}
