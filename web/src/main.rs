use dioxus::prelude::*;

use ui::Navbar;
use views::Home;

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

#[component]
fn App() -> Element {
    rsx! {
        Container {}

        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

#[component]
fn Container() -> Element {
    rsx! {
        div {
            class: "container",
            Sidebar {},
            Main {},
        }
    }
}

#[component]
fn Sidebar() -> Element {
    rsx! {
        div {
            class: "sidebar",
            SearchBar {},
            ChatList {},
        }
    }
}

#[component]
fn SearchBar() -> Element {
    rsx! {
        div {
            class: "search-bar",
            input {
                type: "text",
                placeholder: "Search",
            }
        }
    }
}

#[component]
fn ChatList() -> Element {
    rsx! {
        div {
            class: "chat-list",
            ChatItem {
                title: "John Doe",
                preview: "Hello, how are you?",
                time: "12:00 PM",
            },
        }
    }
}

#[component]
fn ChatInfo(title: String, preview: String, time: String) -> Element {
    rsx! {
        div {
            class: "chat-item",
            div {
                class: "chat-info",
                div {
                    class: "chat-title",
                    "{title}"
                }
                div {
                    class: "chat-preview",
                    "{preview}"
                }
            }
            div {
                class: "chat-time",
                "{time}"
            }
        }
    }
}

#[component]
fn ChatItem(title: String, preview: String, time: String) -> Element {
    rsx! {
        div {
            class: "chat-item",
            div {
                class: "chat-info",
                div {
                    class: "chat-title",
                    "{title}"
                }
                div {
                    class: "chat-preview",
                    "{preview}"
                }
            }
            div {
                class: "chat-time",
                "{time}"
            }
        }
    }
}

#[component]
fn Main() -> Element {
    rsx! {
        div {
            class: "main",
            Chat {
                messages: vec![
                    ("John Doe".to_string(), "Hello, how are you?".to_string()),
                    ("Jane Doe".to_string(), "I'm fine, thank you!".to_string()),
                ],
                on_send: |message| {
                    log::info!("Sending message: {}", message);
                },
            },
        }
    }
}

#[component]
fn Chat(messages: Vec<(String, String)>, on_send: Callback<String>) -> Element {
    rsx! {
        div {
            class: "chat",
            div {
                class: "chat-messages",
                for (author, content) in messages {
                    div {
                        class: "chat-message",
                        div {
                            class: "message-author",
                            "{author}"
                        },
                        div {
                            class: "message-content",
                            "{content}"
                        },
                    }
                },
            },
            MessageInput { on_send: on_send.clone() },
        }
    }
}

#[component]
fn MessageInput(on_send: Callback<String>) -> Element {
    let mut input_value = use_signal(|| "".to_string());

    let handle_send = {
        let mut input_value = input_value.clone();
        move |_| {
            if !input_value().is_empty() {
                on_send(input_value.to_string());
                input_value.set("".to_string());
            }
        }
    };

    rsx! {
        div {
            class: "message-input-container",
            input {
                class: "message-input",
                value: "{input_value()}",
                placeholder: "Type your message...",
                oninput: move |e| input_value.set(e.value().clone()),
            }
            button {
                class: "message-send-button",
                onclick: handle_send,
                "Send"
            }
        }
    }
}
