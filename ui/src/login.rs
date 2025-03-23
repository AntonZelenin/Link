use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_signal;
use dioxus::prelude::*;
use lcore::api::schemas::{AuthError, LoginRequest, RegisterError, RegisterRequest};
use lcore::third_party::utils::form_values_to_string;
use lcore::{auth, utils};
use validator::Validate;
use lcore::api::client::SharedApiClient;
use lcore::traits::SharedStorage;

const LOGIN_CSS: Asset = asset!("/assets/styling/login.css");

#[component]
pub fn LoginModal(is_authenticated: Signal<bool>, show_modal: Signal<bool>) -> Element {
    let mut active_tab = use_signal(|| "login".to_string());

    rsx! {
        document::Link { rel: "stylesheet", href: LOGIN_CSS }
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
    let client = use_context::<SharedApiClient>();
    let storage = use_context::<SharedStorage>();

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

                let client = client.clone();
                let storage = storage.clone();
                spawn(async move {
                    match auth::login(req, client, storage).await {
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
    let client = use_context::<SharedApiClient>();
    let storage = use_context::<SharedStorage>();

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
                let storage = storage.clone();
                spawn(async move {
                    match auth::register(req, client, storage).await {
                        Ok(()) => {
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
