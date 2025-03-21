use crate::views::app::App;
use dioxus::prelude::*;

mod config;
mod logging;
mod storage;
mod views;

fn main() {
    config::init_config();
    logging::init_logger();

    launch(App);
}
