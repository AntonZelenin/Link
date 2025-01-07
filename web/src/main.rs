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
    let selected_chat = use_signal(|| None::<(String, Vec<(String, String)>)>);

    rsx! {
        div {
            class: "container",
            Sidebar { selected_chat: selected_chat }
            MainView { selected_chat: selected_chat }
        }

        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
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
