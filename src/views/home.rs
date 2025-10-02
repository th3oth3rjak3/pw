use crate::{
    components::{Button, ButtonVariant, Input},
    AppState,
};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let mut master_password = use_signal(|| "".to_string());
    let mut username = use_signal(|| "".to_string());

    rsx! {
        form {
            onsubmit: move |_| {
                println!("Username: {username}, password: {master_password}");
            },
            Input {
                name: "username",
                placeholder: "Enter Username",
                r#type: "text",
                value: username,
                value_changed: move |evt: FormEvent| {
                    username.set(evt.value());
                },
            }
            Input {
                name: "master_password",
                placeholder: "Enter Password",
                r#type: "password",
                value: master_password,
                value_changed: move |evt: FormEvent| {
                    master_password.set(evt.value());
                },

            }
            Button { r#type: "submit", variant: ButtonVariant::Secondary, "Submit" }
        }
    }
}
