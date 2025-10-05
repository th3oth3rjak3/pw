use std::sync::Arc;

use crate::{
    components::{Button, ButtonVariant, PasswordInput},
    routes::Route,
    services::{
        authentication::{self, LoginError},
        database::DatabaseService,
    },
    AuthState,
};
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use zeroize::Zeroizing;

/// The Login page component that will be rendered when the current route is `[Route::Login]`
#[component]
pub fn Login() -> Element {
    let state = use_context::<Signal<AuthState>>();
    let db_service = use_context::<Arc<DatabaseService>>();

    // The contents of the password input field
    let mut password = use_signal(|| Zeroizing::new(String::new()));

    // Error message signals
    let error_message = use_signal(|| "".to_string());
    let mut show_error = use_signal(|| false);

    let navigator = use_navigator();
    let toast_api = use_toast();

    let do_login = move || {
        let mut state = state.clone();
        let mut password = password.clone();
        let mut error_message = error_message.clone();
        let mut show_error = show_error.clone();
        let navigator = navigator.clone();
        let db_service = db_service.clone();

        spawn(async move {
            match authentication::login(password(), state(), &db_service).await {
                Ok(updated) => {
                    state.set(updated);
                    password.set(Zeroizing::new(String::new()));
                    navigator.replace(Route::vault());
                }
                Err(e) => match e {
                    LoginError::IncorrectPassword => {
                        error_message.set("incorrect password, try again".into());
                        show_error.set(true);
                    }
                    LoginError::HashingError(err) => toast_api.error(
                        "Error".into(),
                        ToastOptions::new()
                            .description(format!(
                                "Error occurred that requires developer attention: {err}"
                            ))
                            .permanent(true),
                    ),
                },
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
                    do_login();
                },
                div {

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
                    "Login"
                }
            }
        }
    }
}
