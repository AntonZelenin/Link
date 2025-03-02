use dioxus::prelude::{Global, GlobalSignal};
use crate::models::auth::Auth;

pub static AUTH: GlobalSignal<Auth> = Global::new(|| Auth::new());
