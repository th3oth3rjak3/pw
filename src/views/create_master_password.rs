use std::sync::Arc;

use crate::{
    components::{Button, ButtonVariant, PasswordInput},
    routes::Route,
    services::{
        authentication::{self},
        database::DatabaseService,
    },
};
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use zeroize::Zeroizing;

/// The CreateMasterPassword page component that will be rendered when the current route is `[Route::CreateMasterPassword]`
#[component]
pub fn CreateMasterPassword() -> Element {
    let db_service = use_context::<Arc<DatabaseService>>();

    // The contents of the password input field
    let mut password = use_signal(|| Zeroizing::new(String::new()));
    let mut confirm_password = use_signal(|| Zeroizing::new(String::new()));

    // Error message signals
    let mut error_message = use_signal(|| "".to_string());
    let mut show_error = use_signal(|| false);

    let navigator = use_navigator();
    let toast_api = use_toast();

    let set_master_password = move || {
        let mut password = password.clone();
        let navigator = navigator.clone();
        let db_service = db_service.clone();

        spawn(async move {
            match authentication::set_master_password(password(), &db_service).await {
                Ok(_) => {
                    password.set(Zeroizing::new(String::new()));
                    confirm_password.set(Zeroizing::new(String::new()));
                    navigator.replace(Route::login());
                }
                Err(e) => toast_api.error(
                    "Error".into(),
                    ToastOptions::new().description(e).permanent(true),
                ),
            }
        })
    };

    rsx! {
        div { style: "
            display: flex;
            justify-content: center;
            align-items: flex-start;
            height: 100%;
            margin-top: 15vh;
        ",
            form {
                style: "
                display: flex;
                flex-direction: column;
                align-items: flex-start;
                gap: 0.5rem;
                width: 200px;
            ",
                onsubmit: move |_| {
                    show_error.set(false);
                    if password() != confirm_password() {
                        error_message.set("passwords do not match".into());
                        show_error.set(true);
                        return;
                    }
                    set_master_password();
                },
                PasswordInput {
                    style: "width: 200px",
                    name: "master_password",
                    placeholder: "Enter Password",
                    r#type: "password",
                    value: password().to_string(),
                    value_changed: move |evt: FormEvent| {
                        password.set(Zeroizing::new(evt.value()));
                    },
                }
                div {
                    PasswordInput {
                        style: "width: 200px",
                        name: "master_password",
                        placeholder: "Confirm Password",
                        r#type: "password",
                        value: confirm_password().to_string(),
                        value_changed: move |evt: FormEvent| {
                            confirm_password.set(Zeroizing::new(evt.value()));
                        },
                    }
                    if show_error() {
                        div {
                            small { style: "margin-left: 5px; color: var(--primary-error-color)",
                                {error_message}
                            }
                        }
                    }
                }
                Button {
                    style: "width: 200px",
                    r#type: "submit",
                    variant: ButtonVariant::Secondary,
                    "Set Master Password"
                }
            }
        }
    }
}
