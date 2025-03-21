use crate::storage::get_storage;
use dioxus::prelude::*;
use lcore::api::client::{Client, SharedClient};
use views::Home;
use web_sys::js_sys::eval;
use lcore::traits::SharedStorage;
use ui::generic::Sidebar;
use ui::home::MainView;

mod config;
mod logging;
mod storage;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    config::init_config();
    logging::init_logger();

    launch(App);
}

#[component]
fn App() -> Element {
    init_context();

    eval("document.title = '< L ї n k >'").expect("Failed to set document title");

    let is_authenticated = use_signal(|| false);
    let show_login_modal = use_signal(|| true);
    let selected_chat = use_signal(|| None::<(String, Vec<(String, String)>)>);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}

        div {
            class: "container",
            // Only show main UI if authenticated
            {
                if *is_authenticated.read() {
                    rsx! {
                        Sidebar { selected_chat: selected_chat }
                        MainView { selected_chat: selected_chat }
                    }
                } else {
                    rsx! {}
                }
            }

            {
                if *show_login_modal.read() {
                    rsx! {
                        ui::login::LoginModal {
                            is_authenticated: is_authenticated,
                            show_modal: show_login_modal,
                        }
                    }
                } else {
                    rsx! {}
                }
            }
        }
    }
}

fn init_context() {
    let config = config::get_config();
    let client = Client::new(
        None,
        config.core.auth_service_api_url.clone(),
        config.core.user_service_api_url.clone(),
        config.core.message_service_api_url.clone(),
    );
    let shared_client = SharedClient::new(client);
    use_context_provider(|| shared_client);

    let storage = SharedStorage::new(get_storage());
    use_context_provider(|| storage);
}
