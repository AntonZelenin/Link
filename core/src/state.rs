use dioxus::prelude::{Global, GlobalSignal};
use crate::models::auth::Auth;

pub static AUTH: GlobalSignal<Option<Auth>> = Global::new(|| None);
