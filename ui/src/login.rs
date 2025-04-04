use dcore::state::auth::SharedAuthState;
use dcore::utils::form_values_to_string;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_signal;
use dioxus::prelude::*;
use lcore::api::client::SharedApiClient;
use lcore::api::schemas::{AuthError, LoginRequest, RegisterError, RegisterRequest};
use lcore::prelude::*;
use lcore::{auth, utils};
use validator::Validate;

const CSS: Asset = asset!("/assets/styling/login.css");

#[component]
pub fn Login() -> Element {
    let mut active_tab = use_signal(|| "login".to_string());

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }
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
                    LoginForm { }
                }
                if *active_tab.read() == "register" {
                    RegisterForm { }
                }
            }
        }
    }
}

#[component]
pub fn LoginForm() -> Element {
    let auth_state = use_context::<SharedAuthState>();
    let client = use_context::<SharedApiClient>();
    let storage = use_context::<SharedStorage>();

    let mut error = use_signal(|| String::new());
    let mut processing = use_signal(|| false);

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

                let auth_state = auth_state.clone();
                let client = client.clone();
                let storage = storage.clone();
                spawn(async move {
                    match auth::login(req, client, storage, auth_state).await {
                        Ok(()) => {}
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
pub fn RegisterForm() -> Element {
    let auth_state = use_context::<SharedAuthState>();
    let client = use_context::<SharedApiClient>();
    let storage = use_context::<SharedStorage>();

    let mut error_password = use_signal(|| String::new());
    let mut error_username = use_signal(|| String::new());
    let mut processing = use_signal(|| false);

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

                let auth_state = auth_state.clone();
                let client = client.clone();
                let storage = storage.clone();
                spawn(async move {
                    match auth::register(req, client, storage, auth_state).await {
                        Ok(()) => {}
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
