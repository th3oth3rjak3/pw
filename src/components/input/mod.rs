use dioxus::prelude::*;

use crate::components::{EyeClosed, EyeOpen};

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
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/input/style.css"),
        }
        input { class: "input", oninput: props.value_changed, ..props.attributes, {props.children} }
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
            ",

            // Reuse your existing Input component
            Input {
                value_changed: props.value_changed.clone(),
                r#type: if show_password() { "text" } else { "password" },
                style: "padding-right: 2.5rem;",
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
