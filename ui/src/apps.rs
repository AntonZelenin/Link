use crate::messenger::MessengerApp;
use dioxus::prelude::*;
use lcore::api::client::SharedApiClient;
use lcore::api::schemas::AuthError;
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
    let error = use_signal::<Option<String>>(|| None);
    let storage = use_context::<SharedStorage>();
    let client = use_context::<SharedApiClient>();

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

                    if let Some(msg) = &*error.read() {
                        div {
                            class: "menu-error",
                            "{msg}"
                        }
                    }

                    button {
                        class: "menu-button",
                        onclick: move |_| {
                            let client = client.clone();
                            let storage = storage.clone();
                            let mut error = error.to_owned();
                            spawn(async move {
                                match auth::logout(client, storage).await {
                                    Ok(_) => {}
                                    Err(err) => {
                                        match err {
                                            AuthError::ApiError(msg) => {
                                                error.set(Some(msg));
                                            }
                                        }
                                    }
                                }
                            });
                        },
                        "Log out"
                    }
                }
            }
        }
    }
}
