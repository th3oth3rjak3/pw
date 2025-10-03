use crate::{
    components::{Button, ButtonVariant, PasswordInput},
    routes::Route,
    services::authentication::{self, LoginError},
    AppState,
};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let mut state = use_context::<Signal<AppState>>();

    // The contents of the password input field
    let mut password = use_signal(|| "".to_string());

    // Error message signals
    let mut error_message = use_signal(|| "".to_string());
    let mut show_error = use_signal(|| false);

    let navigator = use_navigator();

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
                    let raw_pw = password.read().as_bytes().to_vec();
                    let owned_state = state.read().to_owned();
                    match authentication::login(raw_pw, owned_state) {
                        Ok(updated) => {
                            state.set(updated);
                            password.set("".into());
                            navigator.replace(Route::vault());
                        }
                        Err(e) => {
                            match e {
                                LoginError::IncorrectPassword => {
                                    error_message.set("incorrect password, try again".into());
                                    show_error.set(true);
                                }
                            }
                        }
                    }
                },
                div {

                    PasswordInput {
                        style: "width: 200px",
                        name: "master_password",
                        placeholder: "Enter Password",
                        r#type: "password",
                        value: password,
                        value_changed: move |evt: FormEvent| {
                            password.set(evt.value());
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
