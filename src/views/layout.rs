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
        if signed_in() {
            Navbar { style: "margin-bottom: 5px",
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

        div { style: "padding: 0.1rem 0.5rem 0.5rem 0.5rem;", Outlet::<Route> {} }
    }
}
