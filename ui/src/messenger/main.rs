use crate::messenger::messenger::{MessengerConversationArea, Sidebar};
use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/messenger/main.css");

#[component]
pub fn Messenger() -> Element {
    let selected_chat = use_signal(|| None::<(String, Vec<(String, String)>)>);

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        div {
            class: "messenger-container",
            Sidebar { selected_chat: selected_chat }
            MessengerConversationArea { selected_chat: selected_chat }
        }
    }
}
