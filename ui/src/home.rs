use crate::messenger::Chat;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn MainView(selected_chat: Signal<Option<(String, Vec<(String, String)>)>>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

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
