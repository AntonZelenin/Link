use dioxus::prelude::*;
use crate::messenger::MessengerApp;

const CSS: Asset = asset!("/assets/styling/apps.css");

#[component]
pub fn AppsView() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        div {
            class: "apps-container",

            MessengerApp {}
        }
    }
}
