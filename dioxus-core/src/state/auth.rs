use dioxus::prelude::*;
use lcore::traits::AuthState;
use std::sync::{Arc, RwLock};

pub static IS_AUTHENTICATED: GlobalSignal<bool> = Global::new(|| false);

#[derive(Clone)]
pub struct SharedAuthState(Arc<RwLock<DioxusAuthState>>);

impl SharedAuthState {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(DioxusAuthState)))
    }
}

impl AuthState for SharedAuthState {
    fn set_authenticated(&self) {
        self.0.write().unwrap().set_authenticated();
    }

    fn set_not_authenticated(&self) {
        self.0.write().unwrap().set_not_authenticated();
    }

    fn is_authenticated(&self) -> bool {
        self.0.read().unwrap().is_authenticated()
    }
}

pub struct DioxusAuthState;

impl DioxusAuthState {
    fn set_authenticated(&self) {
        *IS_AUTHENTICATED.write() = true;
    }

    fn set_not_authenticated(&self) {
        *IS_AUTHENTICATED.write() = false;
    }

    fn is_authenticated(&self) -> bool {
        *IS_AUTHENTICATED.read()
    }
}
