use dioxus::prelude::*;

use crate::{
    components::{EyeClosed, EyeOpen},
    models::AuthState,
};

#[derive(Debug, Clone, Props, PartialEq)]
pub struct InputProps {
    #[props(into, optional)]
    value_changed: Callback<Event<FormData>>,
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let mut state = use_context::<Signal<AuthState>>();

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/input/style.css"),
        }
        input {
            class: "input",
            oninput: move |evt| {
                state.write().reset_idle_timer();
                props.value_changed.call(evt);
            },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Debug, Clone, Props, PartialEq)]
pub struct PasswordInputProps {
    #[props(into, optional)]
    pub value_changed: Callback<Event<FormData>>,
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn PasswordInput(props: PasswordInputProps) -> Element {
    let mut show_password = use_signal(|| false);

    rsx! {
        div { style: "
                position: relative;
                display: flex;
                align-items: center;
                width: 100%;
                max-width: 100%;
            ",

            // Reuse your existing Input component
            Input {
                value_changed: props.value_changed,
                r#type: if show_password() { "text" } else { "password" },
                style: "
                    width: 100%;
                    box-sizing: border-box;
                    padding-right: 2.5rem;
                ",
                attributes: props.attributes,
            }

            button {
                r#type: "button",
                onclick: move |_| show_password.set(!show_password()),
                style: "
                    position: absolute;
                    right: 0.2rem;
                    top: 55%;
                    transform: translateY(-50%);
                    background: none;
                    border: none;
                    cursor: pointer;
                    font-size: 1rem;
                    color: #666;
                ",
                {if show_password() { EyeOpen() } else { EyeClosed() }}
            }
        }
    }
}
