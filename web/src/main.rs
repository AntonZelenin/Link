use crate::storage::get_storage;
use dioxus::prelude::*;
use js_sys::eval;
use lcore::prelude::*;
use ui::messenger;

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

    register_apps_from_config(storage.clone());
}

pub fn register_apps_from_config(storage: SharedStorage) {
    let config = config::get_config();

    // for now, they are hardcoded, I'll move to dlls later
    if config.core.apps.is_app_enabled(messenger::NAME) {
        register_app(messenger::NAME, messenger::Messenger);
    }

    load_active_app(storage);
}
