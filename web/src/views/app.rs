use crate::storage::get_storage;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use js_sys::eval;
use lcore::prelude::*;
use lcore::traits::SharedStorage;

const CSS: Asset = asset!("/assets/styling/main.css");

#[component]
pub fn App() -> Element {
    init();

    eval("document.title = '< L ї n k >'").expect("Failed to set document title");

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        div {
            class: "app-container",
            {
                if *IS_AUTHENTICATED.read() {
                    rsx! {
                        ui::messenger::Messenger {}
                    }
                } else {
                    rsx! {
                        ui::login::Login {}
                    }
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
