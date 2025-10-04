use crate::{
    components::{Navbar, NavbarItem},
    services::authentication,
    AuthState, Route,
};
use dioxus::prelude::*;

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of all routes in a common navbar. The contents of the page
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Layout() -> Element {
    let mut state = use_context::<Signal<AuthState>>();

    let signed_in = state.map(|s| &s.signed_in);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/navbar/style.css"),
        }
        Navbar { style: "margin-bottom: 5px",
            if signed_in() {
                NavbarItem {
                    index: 1usize,
                    value: "vault".to_string(),
                    to: Route::vault(),
                    disabled: !state.read().signed_in,
                    "Vault"
                }

                NavbarItem {
                    index: 10usize,
                    style: "margin-left: auto",
                    value: "logout".to_string(),
                    to: Route::home(),
                    onclick: move |_| {
                        state.set(authentication::logout());
                    },
                    "Logout"
                }
            }
        }

        div { style: "margin: 10px;", Outlet::<Route> {} }
        if cfg!(debug_assertions) {
            div { style: "position: fixed; bottom: 0; width: 100%;",
                hr {}
                "DEBUG INFORMATION:"
                div { {format!("{:#?}", state.read())} }
            }
        }
    }
}
