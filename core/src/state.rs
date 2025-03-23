use dioxus::prelude::{Global, GlobalSignal};

pub static IS_AUTHENTICATED: GlobalSignal<bool> = Global::new(|| false);
