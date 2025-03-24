use crate::messenger::MessengerApp;
use dioxus::prelude::*;
use lcore::auth;
use lcore::prelude::SharedStorage;

const CSS: Asset = asset!("/assets/styling/apps.css");

#[component]
pub fn AppsView() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        div {
            class: "apps-container",

            div {
                class: "apps-list",
                // hardcoded for now, will load dynamically later
                MessengerApp {}
            }

            div {
                class: "menu-section",
                Menu{}
            }
        }
    }
}

#[component]
pub fn Menu() -> Element {
    let mut show_menu = use_signal(|| false);
    let storage = use_context::<SharedStorage>();

    rsx! {
        div {
            class: "menu-icon-wrapper",

            div {
                class: "menu-icon",
                onclick: move |_| show_menu.set(!show_menu()),
                "☰"
            }

            if show_menu() {
                div {
                    class: "menu-popup",
                    button {
                        class: "menu-button",
                        onclick: move |_| {
                            auth::logout(storage.clone());
                        },
                        "Log out"
                    }
                }
            }
        }
    }
}
