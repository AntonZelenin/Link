use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::{use_resource, use_signal};
use dioxus::prelude::*;
use lcore::api::schemas::{AuthError, AuthResponse, LoginRequest, RegisterRequest};
use lcore::third_party::utils::form_values_to_string;
use lcore::traits::ToJson;
use lcore::utils;
use wasm_bindgen::prelude::*;

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
                spawn(async move {
                    match login(req).await {
                        Ok(auth_data) => {
                            web_sys::console::log_1(&"ok".into());
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
                        Err(e) => {
                            web_sys::console::log_1(&"error".into());
                            web_sys::console::log_1(&e.clone().into());
                            error.set(e);
                        }
                    }
                    processing.set(false);
                });
            },
            class: "login-modal-form",

            input {
                class: "login-modal-input",
                "type": "text",
                placeholder: "Username",
                name: "username"
            }

            input {
                class: "login-modal-input",
                "type": "password",
                placeholder: "Password",
                name: "password"
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
    let mut error = use_signal(|| String::new());
    let mut processing = use_signal(|| false);

    rsx! {
        form {
            onsubmit: move |ev| {
                processing.set(true);
                error.set("".to_string());
                let req = utils::from_map::<RegisterRequest>(
                    &form_values_to_string(&ev.values())
                ).unwrap();
                let auth_result = use_resource(move || {
                    let req_clone = req.clone();
                    async move {
                        register(req_clone).await
                    }
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
                name: "username"
            }

            input {
                class: "login-modal-input",
                "type": "password",
                placeholder: "Password",
                name: "password"
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
                    } else {
                        "Register"
                    }}
                }
            }
        }
    }
}
