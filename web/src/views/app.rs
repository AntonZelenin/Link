use crate::storage::get_storage;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::{use_context_provider, use_signal};
use dioxus::prelude::*;
use js_sys::eval;
use lcore::prelude::*;
use lcore::traits::SharedStorage;
use ui::generic::Sidebar;
use ui::home::MainView;

const CSS: Asset = asset!("/assets/styling/main.css");

#[component]
pub fn App() -> Element {
    init();

    eval("document.title = '< L ї n k >'").expect("Failed to set document title");

    let selected_chat = use_signal(|| None::<(String, Vec<(String, String)>)>);

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        div {
            class: "container",
            // Only show main UI if authenticated
            {
                if *IS_AUTHENTICATED.read() {
                    rsx! {
                        Sidebar { selected_chat: selected_chat }
                        MainView { selected_chat: selected_chat }
                    }
                } else {
                    rsx! {}
                }
            }

            {
                if !*IS_AUTHENTICATED.read() {
                    rsx! {
                        ui::login::LoginModal {}
                    }
                } else {
                    rsx! {}
                }
            }
        }
    }
}

fn init() {
    let storage = SharedStorage::new(get_storage());
    use_context_provider(|| storage.clone());

    let auth_manager = lcore::auth::factory::get_auth_manager(storage.clone());
    *IS_AUTHENTICATED.write() = auth_manager.is_authenticated();

    let shared_client = lcore::api::factory::get_shared_api_client(storage.clone());
    use_context_provider(|| shared_client);
}
