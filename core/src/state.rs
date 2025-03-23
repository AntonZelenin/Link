use dioxus::prelude::{Global, GlobalSignal};
use crate::auth::schemas::Auth;

pub static AUTH: GlobalSignal<Option<Auth>> = Global::new(|| None);
