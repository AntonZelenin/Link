use dioxus::prelude::*;

#[component]
pub fn Border() -> Element {
    rsx! {
        div { class: "border" }
    }
}

#[component]
pub fn ShortBorder() -> Element {
    rsx! {
        div { class: "short-border" }
    }
}
