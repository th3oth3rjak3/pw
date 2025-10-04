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
pub fn PasswordDetails(id: i32) -> Element {
    let auth_state = use_context::<Signal<AuthState>>();
    let db_service = use_context::<Arc<DatabaseService>>();
    let db_service = use_signal(|| db_service.clone());
    let navigator = use_navigator();
    let toast_api = use_toast();

    if !auth_state().signed_in {
        navigator.replace(Route::home());
    }

    let mut site = use_signal(|| "".to_string());
    let mut username = use_signal(|| "".to_string());
    let mut raw_password = use_signal(|| "".to_string());

    let mut new_site = use_signal(|| "".to_string());
    let mut new_username = use_signal(|| "".to_string());
    let mut new_raw_password = use_signal(|| "".to_string());

    let entry = use_resource(move || {
        let auth_state = auth_state().clone();
        let db_service = db_service.clone();

        async move {
            match password_entry::get_password_entry_by_id(id, &auth_state, db_service().as_ref())
                .await
            {
                Ok(pw) => {
                    site.set(pw.site.clone());
                    username.set(pw.username.clone());
                    raw_password.set(pw.raw_password.clone());
                    new_site.set(pw.site);
                    new_username.set(pw.username);
                    new_raw_password.set(pw.raw_password);
                }
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
        }
    });

    let save_pw = move |password: PasswordEntryRaw| {
        // clone needed signals here
        let auth_state = auth_state().clone();
        let db_service = db_service.clone();

        spawn(async move {
            if let Err(err) = password_entry::save_updated_password(
                id,
                password,
                &auth_state,
                db_service().as_ref(),
            )
            .await
            {
                toast_api.error(
                    "Error".into(),
                    ToastOptions::new()
                        .description(format!(
                            "Error occurred that requires developer attention: {err}"
                        ))
                        .permanent(true),
                )
            }
        });
    };

    let mut editing_password = use_signal(|| false);

    match entry() {
        Some(_) => rsx! {
            div { style: "display: flex; justify-content: center; padding: 0;",

                Card { title: "Password Details",

                    form { style: "display: flex; flex-direction: column",
                        FieldGroup {
                            Field { label: "Site",
                                Input {
                                    name: "site",
                                    placeholder: "Site",
                                    value: new_site(),
                                    value_changed: move |evt: FormEvent| new_site.set(evt.value()),
                                    readonly: !editing_password(),
                                }
                            }

                            Field { label: "Username",
                                Input {
                                    name: "username",
                                    placeholder: "Username",
                                    value: new_username(),
                                    value_changed: move |evt: FormEvent| new_username.set(evt.value()),
                                    readonly: !editing_password(),
                                }
                            }
                            Field { label: "Password",
                                PasswordInput {
                                    name: "password",
                                    placeholder: "Password",
                                    value: new_raw_password(),
                                    value_changed: move |evt: FormEvent| new_raw_password.set(evt.value()),
                                    readonly: !editing_password(),
                                }
                            }
                        }
                    }


                    div { style: "display: flex; justify-content: flex-end; gap: 0.3rem; margin: 0;",
                        if !editing_password() {
                            Button {
                                variant: ButtonVariant::Ghost,
                                onclick: move |_| editing_password.set(true),
                                "Edit"
                            }
                        }
                        if editing_password() {
                            Button {
                                variant: ButtonVariant::Ghost,
                                onclick: move |_| {
                                    save_pw(PasswordEntryRaw {
                                        id,
                                        site: new_site(),
                                        username: new_username(),
                                        raw_password: new_raw_password(),
                                    });
                                    editing_password.set(false);
                                },
                                "Save"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                onclick: move |_| {
                                    new_site.set(site());
                                    new_username.set(username());
                                    new_raw_password.set(raw_password());
                                    editing_password.set(false);
                                },
                                "Cancel"
                            }
                        }
                    }
                }
            }
        },
        _ => rsx! {},
    }
}
