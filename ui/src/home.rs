use crate::apps::AppsView;
use crate::login;
use dcore::state::app::get_active_app;
use dcore::state::auth::SharedAuthState;
use dioxus::prelude::*;
use lcore::prelude::*;
use manganis::asset;

const CSS: Asset = asset!("/assets/styling/home.css");
const GENERIC_CSS: Asset = asset!("/assets/styling/generic.css");

#[component]
pub fn App() -> Element {
    let auth_state = use_context::<SharedAuthState>();

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }
        document::Link { rel: "stylesheet", href: GENERIC_CSS }

        div {
            class: "app-container",
            {
                if auth_state.is_authenticated() {
                    rsx! {
                        AppView {}
                        AppsView {}
                    }
                } else {
                    rsx! {
                        login::Login {}
                    }
                }
            }
        }
    }
}

#[component]
fn AppView() -> Element {
    if let Some(app) = get_active_app() {
        let App = app;
        return rsx! {
            App {}
        };
    }

    rsx! { Empty {} }
}

#[component]
fn Empty() -> Element {
    rsx! {
        div {
            class: "empty-app-container",

            "Choose an app to start working"
        }
    }
}
