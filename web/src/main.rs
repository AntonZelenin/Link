use dioxus::prelude::*;

use ui::Navbar;
use views::Home;

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Container {}

        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

#[component]
fn Container() -> Element {
    rsx! {
        div {
            class: "container",
            Sidebar {},
            Main {},
        }
    }
}

#[component]
fn Sidebar() -> Element {
    rsx! {
        div {
            class: "sidebar",
            SearchBar {},
            ChatList {},
        }
    }
}

#[component]
fn SearchBar() -> Element {
    rsx! {
        div {
            class: "search-bar",
            input {
                type: "text",
                placeholder: "Search",
            }
        }
    }
}

#[component]
fn ChatList() -> Element {
    rsx! {
        div {
            class: "chat-list"
        }
    }
}

#[component]
fn Main() -> Element {
    rsx! {
        div {
            class: "main"
        }
    }
}
