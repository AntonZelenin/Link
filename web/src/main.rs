use dioxus::prelude::*;
use dioxus::web::launch::launch_cfg;
use dioxus::web::Config;
use serde::{Deserialize, Serialize};
use ui::Navbar;
use views::Home;
use web_sys::js_sys::eval;

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    launch(App);
}

// API response structures
#[derive(Debug, Serialize, Deserialize)]
struct AuthResponse {
    access_token: String,
    refresh_token: String,
    user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthError {
    detail: String,
}

#[component]
fn App() -> Element {
    // eval("document.title = 'O r B ї t'").expect("Failed to set document title");
    eval("document.title = '< l ї n k >'").expect("Failed to set document title");

    let is_authenticated = use_signal(|| false);
    let show_login_modal = use_signal(|| true);
    let selected_chat = use_signal(|| None::<(String, Vec<(String, String)>)>);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}

        div {
            class: "container",
            // Only show main UI if authenticated
            {
                if *is_authenticated.read() {
                    rsx! {
                        Sidebar { selected_chat: selected_chat }
                        MainView { selected_chat: selected_chat }
                    }
                } else {
                    rsx! {}
                }
            }

            {
                if *show_login_modal.read() {
                    rsx! {
                        LoginModal {
                            is_authenticated: is_authenticated,
                            show_modal: show_login_modal,
                        }
                    }
                } else {
                    rsx! {}
                }
            }
        }
    }
}

#[component]
fn LoginModal(is_authenticated: Signal<bool>, show_modal: Signal<bool>) -> Element {
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
                    // "Welcome to OrBїt"
                    "Welcome to Lїnk"
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

#[component]
fn Sidebar(selected_chat: Signal<Option<(String, Vec<(String, String)>)>>) -> Element {
    rsx! {
        div { class: "sidebar",
            SearchBar {}
            ChatList { selected_chat: selected_chat }
        }
    }
}

#[component]
fn SearchBar() -> Element {
    rsx! {
        div { class: "search-bar",
            input {
                "type": "text",
                placeholder: "Search"
            }
        }
    }
}

#[component]
fn ChatList(selected_chat: Signal<Option<(String, Vec<(String, String)>)>>) -> Element {
    let chats = vec![
        (
            "John Doe".to_string(),
            vec![("John".to_string(), "Hello!".to_string())],
        ),
        (
            "Jane Smith".to_string(),
            vec![("Jane".to_string(), "Hi there!".to_string())],
        ),
    ];

    rsx! {
        div { class: "chat-list",
            {chats.into_iter().map(|(title, messages)| {
                let title_clone = title.clone();
                let messages_clone = messages.clone();
                rsx! {
                    ChatItem {
                        key: "{title}",
                        title: title_clone,
                        preview: messages_clone.first().map_or("".to_string(), |(_, content)| content.clone()),
                        time: "14:32".to_string(),
                        on_click: move |_| selected_chat.set(Some((title.clone(), messages.clone())))
                    }
                    ShortBorder {}
                }
            })}
        }
    }
}

#[component]
fn ChatItem(title: String, preview: String, time: String, on_click: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "chat-item",
            onclick: move |_| on_click.call(()),
            div { class: "chat-info",
                div { class: "chat-title",
                    "{title}"
                }
                div { class: "chat-preview",
                    "{preview}"
                }
            }
            div { class: "chat-time",
                "{time}"
            }
        }
    }
}

#[component]
fn MainView(selected_chat: Signal<Option<(String, Vec<(String, String)>)>>) -> Element {
    rsx! {
        div {
            class: "main",
            {
                match &*selected_chat.read() {
                    Some((title, messages)) => rsx! {
                        Chat {
                            title: title.clone(),
                            messages: messages.clone(),
                            on_send: move |msg: String| {
                                println!("Sending message: {}", msg);
                            }
                        }
                    },
                    None => rsx! {
                        div {
                            class: "no-chat-selected",
                            "Select a chat to start messaging!"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Chat(title: String, messages: Vec<(String, String)>, on_send: EventHandler<String>) -> Element {
    rsx! {
        div { class: "chat visible",
            div { class: "chat-header",
                "{title}"
            }
            div { class: "chat-messages",
                {messages.into_iter().map(|(author, content)| {
                    rsx! {
                        div {
                            key: "{author}-{content}",
                            class: "chat-message",
                            div { class: "message-author",
                                "{author}"
                            }
                            div { class: "message-content",
                                "{content}"
                            }
                        }
                    }
                })}
            }
            MessageInput {
                on_send: on_send
            }
        }
    }
}

#[component]
fn MessageInput(on_send: EventHandler<String>) -> Element {
    let mut input_value = use_signal(String::new);

    rsx! {
        div { class: "message-input-container",
            input {
                class: "message-input",
                value: "{input_value.read()}",
                placeholder: "Type your message...",
                oninput: move |evt| input_value.set(evt.value().clone())
            }
            button {
                class: "message-send-button",
                onclick: move |_| {
                    if !input_value.read().is_empty() {
                        on_send.call(input_value.read().clone());
                        input_value.set(String::new());
                    }
                },
                "Send"
            }
        }
    }
}

#[component]
fn ShortBorder() -> Element {
    rsx! {
        div { class: "short-border" }
    }
}
