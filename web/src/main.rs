use crate::storage::get_storage;
use dioxus::prelude::*;
use js_sys::eval;
use lcore::prelude::IS_AUTHENTICATED;
use lcore::traits::SharedStorage;

mod config;
mod logging;
mod storage;

const CSS: Asset = asset!("/assets/styling/web-app.css");

fn main() {
    config::init_config();
    logging::init_logger();

    launch(WebApp);
}

#[component]
pub fn WebApp() -> Element {
    init();

    eval("document.title = '< L ї n k >'").expect("Failed to set document title");

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        ui::home::App {}
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
