use crate::config;
use crate::storage::get_storage;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::{use_context_provider, use_signal};
use dioxus::prelude::*;
use js_sys::eval;
use lcore::api::client::{ApiClient, SharedApiClient};
use lcore::traits::SharedStorage;
use ui::generic::Sidebar;
use ui::home::MainView;

const CSS: Asset = asset!("/assets/styling/main.css");

#[component]
pub fn App() -> Element {
    init_context();

    eval("document.title = '< L ї n k >'").expect("Failed to set document title");

    let is_authenticated = use_signal(|| false);
    let show_login_modal = use_signal(|| true);
    let selected_chat = use_signal(|| None::<(String, Vec<(String, String)>)>);

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

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
    let storage = SharedStorage::new(get_storage());
    use_context_provider(|| storage.clone());

    let shared_client = lcore::api::factory::get_shared_api_client(storage.clone());
    use_context_provider(|| shared_client);
}
