use crate::messenger::ChatList;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/generic.css");

#[component]
pub fn Sidebar(selected_chat: Signal<Option<(String, Vec<(String, String)>)>>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        div { class: "sidebar",
            SearchBar {}
            ChatList { selected_chat: selected_chat }
        }
    }
}

#[component]
pub fn SearchBar() -> Element {
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
pub fn ShortBorder() -> Element {
    rsx! {
        div { class: "short-border" }
    }
}
