use std::{sync::Arc, time::Duration};

use dioxus::prelude::*;
use dioxus_primitives::{
    scroll_area::ScrollDirection,
    toast::{use_toast, ToastOptions},
};
use zeroize::Zeroizing;

use crate::{
    components::{Button, ButtonVariant, Card, Input, ScrollArea},
    models::{AuthState, PasswordEntryRaw},
    routes::Route,
    services::{clipboard, database::DatabaseService, password_entry},
};

#[component]
pub fn Vault() -> Element {
    let auth_state = use_context::<Signal<AuthState>>();
    let db_service = use_context::<Arc<DatabaseService>>();
    let db_service = use_signal(|| db_service.clone());
    let navigator = use_navigator();
    let toast_api = use_toast();

    if !auth_state().signed_in {
        navigator.replace(Route::home());
    }

    let mut entries: Signal<Vec<PasswordEntryRaw>> = use_signal(Vec::new);
    let mut search_string = use_signal(|| "".to_string());

    let search = move || async move {
        match password_entry::get_all_password_entries(
            &auth_state(),
            db_service().as_ref(),
            search_string(),
        )
        .await
        {
            Ok(pws) => entries.set(pws),
            Err(err) => {
                toast_api.error(
                    "Error".to_string(),
                    ToastOptions::new()
                        .description(format!(
                            "Unexpected error occurred while getting password entries: {err}"
                        ))
                        .permanent(true),
                );
            }
        }
    };

    use_future(search);

    rsx! {
        div { style: "width: 100%; display: flex; justify-content: center;",
            Card {
                title: "Password Vault",
                width: "100%",
                height: "calc(100vh - 110px)",
                div { style: "display: flex; align-items: center; gap: 1rem; margin-bottom: 0.5rem;",

                    // Left: Add Password button
                    Button {
                        variant: ButtonVariant::Ghost,
                        onclick: move |_| {
                            navigator.push(Route::new_password_entry());
                        },
                        "Add Password"
                    }

                    // Right: Search input
                    form {
                        style: "display: flex; margin-left: auto;",
                        onsubmit: move |_| {
                            spawn(async move { search().await });
                        },
                        Input {
                            name: "search",
                            placeholder: "Search",
                            value: search_string(),
                            value_changed: move |evt: FormEvent| {
                                search_string.set(evt.value());
                            },
                            style: "width: 200px;",
                        }
                    }
                }

                ScrollArea {
                    height: "calc(100vh - 250px)",
                    min_height: "200px",
                    padding: "0 1.2em 1.2em 1.2em",
                    direction: ScrollDirection::Vertical,
                    tabindex: "0",
                    style: "
                        border: 1px solid #444;
                        border-radius: 12px;
                        background-color: #1b1b1b;
                    ",
                    div { class: "scroll-content", style: "padding-top: 15px;",
                        for entry in entries().iter() {
                            PasswordEntryCard {
                                id: entry.id,
                                site: entry.site.clone(),
                                username: entry.username.clone(),
                                password: entry.raw_password.clone(),
                            }
                        }

                    }
                }
            }
        }
    }
}

#[component]
fn PasswordEntryCard(
    id: i32,
    site: String,
    username: String,
    password: Zeroizing<String>,
) -> Element {
    let mut state = use_context::<Signal<AuthState>>();
    let navigator = use_navigator();
    let mut show_password = use_signal(|| false);
    let password = use_signal(|| password);
    let toast_api = use_toast();

    rsx! {
        div {
            style: "
                background: #1e1e1e;
                border: 1px solid #2a2a2a;
                border-radius: 12px;
                padding: 0.8rem 1.2rem;
                margin-bottom: 0.7rem;
                display: flex;
                flex-direction: column;
                gap: 0.4rem;
                box-shadow: 0 4px 16px rgba(0, 0, 0, 0.25);
                cursor: pointer;
                transition: background 0.15s, border 0.15s;
            ",
            onclick: move |_| {
                state.write().reset_idle_timer();
                navigator.push(Route::password_details(id));
            },

            // Title (site name)
            div { style: "display: flex; justify-content: space-between; align-items: center;",
                strong { style: "font-size: 1rem; color: #f0f0f0;", "{site}" }
            }

            // Username row
            div { style: "display: flex; justify-content: space-between; align-items: center; font-size: 0.9rem; color: #ccc;",
                div { style: "font-weight: 500; min-width: 80px;", "Username:" }
                div { style: "flex: 1; overflow: hidden; text-overflow: ellipsis;",
                    "{username}"
                }
            }

            // Password row
            div { style: "display: flex; justify-content: space-between; align-items: center; font-size: 0.9rem; color: #ccc;",
                div { style: "font-weight: 500; min-width: 80px;", "Password:" }
                div { style: "flex: 1; overflow: hidden; text-overflow: ellipsis;",
                    if show_password() {
                        {password().to_string()}
                    } else {
                        "••••••••"
                    }
                }
            }

            // Buttons row
            div { style: "display: flex; gap: 0.4rem; justify-content: flex-end;",
                Button {
                    variant: ButtonVariant::Ghost,
                    style: "width: 70px; min-width: 70px;",
                    onclick: move |evt: Event<MouseData>| {
                        evt.stop_propagation();
                        show_password.set(!show_password());
                    },
                    if show_password() {
                        "Hide"
                    } else {
                        "Show"
                    }
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    style: "width: 70px; min-width: 70px;",
                    onclick: move |evt: Event<MouseData>| {
                        evt.stop_propagation();
                        let message = clipboard::copy_with_timeout(password().clone(), 5);
                        toast_api
                            .success(
                                "Copied!".into(),
                                ToastOptions::new()
                                    .description(&message)
                                    .duration(Duration::from_secs(5)),
                            )
                    },
                    "Copy"
                }
            }
        }
    }
}
