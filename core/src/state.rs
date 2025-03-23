// todo dioxus in core!
use dioxus::prelude::*;

type AppComponent = Box<dyn Fn() -> Element>;

pub static IS_AUTHENTICATED: GlobalSignal<bool> = Global::new(|| false);
pub static ACTIVE_APP: GlobalSignal<Option<AppComponent>> = Global::new(|| None);
