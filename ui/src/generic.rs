use dioxus::prelude::*;

#[component]
pub fn ShortBorder() -> Element {
    rsx! {
        div { class: "short-border" }
    }
}
