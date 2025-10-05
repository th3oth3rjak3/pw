use std::time::Duration;

use crate::{
    components::{Navbar, NavbarItem},
    services::authentication,
    AuthState, Route,
};
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of all routes in a common navbar. The contents of the page
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Layout() -> Element {
    let mut state = use_context::<Signal<AuthState>>();
    let signed_in = state.map(|s| &s.signed_in);
    let navigator = use_navigator();
    let toast_api = use_toast();

    use_future(move || {
        let mut state = state.clone();
        async move {
            loop {
                tokio::time::sleep(Duration::from_secs(10)).await;
                if state.read().signed_in && state.read().is_expired() {
                    navigator.replace(Route::home());
                    state.set(authentication::logout());
                    toast_api.info(
                        "Logged Out".into(),
                        ToastOptions::new().description("🔒 Logged out due to inactivity."),
                    );
                }
            }
        }
    });

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
                    "Vault"
                }

                NavbarItem {
                    index: 2usize,
                    value: "set master password".to_string(),
                    to: Route::create_master_password(),
                    "Set Master Password"
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
