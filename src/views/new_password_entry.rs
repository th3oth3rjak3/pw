use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

use crate::{
    components::{Button, ButtonVariant, Card, Field, FieldGroup, Input, PasswordInput},
    models::{AuthState, PasswordEntryRaw},
    routes::Route,
    services::{database::DatabaseService, password_entry},
};

#[component]
pub fn NewPasswordEntry() -> Element {
    let auth_state = use_context::<Signal<AuthState>>();
    let db_service = use_context::<Arc<DatabaseService>>();
    let db_service = use_signal(|| db_service.clone());
    let navigator = use_navigator();
    let toast_api = use_toast();

    if !auth_state().signed_in {
        navigator.replace(Route::home());
    }

    let mut new_site = use_signal(|| "".to_string());
    let mut new_username = use_signal(|| "".to_string());
    let mut new_raw_password = use_signal(|| "".to_string());

    let save_pw = move |password: PasswordEntryRaw| {
        // clone needed signals here
        let auth_state = auth_state().clone();
        let db_service = db_service.clone();

        spawn(async move {
            match password_entry::create_password_entry(
                password,
                &auth_state,
                db_service().as_ref(),
            )
            .await
            {
                Ok(()) => {
                    navigator.replace(Route::vault());
                }
                Err(err) => {
                    toast_api.error(
                        "Error".into(),
                        ToastOptions::new()
                            .description(format!(
                                "Error occurred that requires developer attention: {err}"
                            ))
                            .permanent(true),
                    );
                }
            }
        });
    };

    rsx! {
        div { style: "display: flex; justify-content: center; padding: 0;",

            Card { title: "Create Password",

                form { style: "display: flex; flex-direction: column",
                    FieldGroup {
                        Field { label: "Site",
                            Input {
                                name: "site",
                                placeholder: "Site",
                                value: new_site(),
                                value_changed: move |evt: FormEvent| new_site.set(evt.value()),
                            }
                        }

                        Field { label: "Username",
                            Input {
                                name: "username",
                                placeholder: "Username",
                                value: new_username(),
                                value_changed: move |evt: FormEvent| new_username.set(evt.value()),
                            }
                        }
                        Field { label: "Password",
                            PasswordInput {
                                name: "password",
                                placeholder: "Password",
                                value: new_raw_password(),
                                value_changed: move |evt: FormEvent| new_raw_password.set(evt.value()),
                            }
                        }
                    }
                }


                div { style: "display: flex; justify-content: flex-end; gap: 0.3rem; margin: 0;",
                    Button {
                        variant: ButtonVariant::Ghost,
                        onclick: move |_| {
                            save_pw(PasswordEntryRaw {
                                id: 0,
                                site: new_site(),
                                username: new_username(),
                                raw_password: new_raw_password(),
                            });
                        },
                        "Save"
                    }
                    Button {
                        variant: ButtonVariant::Ghost,
                        onclick: move |_| {
                            navigator.replace(Route::vault());
                        },
                        "Cancel"
                    }
                }
            }
        }
    }
}
