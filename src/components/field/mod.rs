use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FieldProps {
    pub label: String,
    pub children: Element,
}

#[component]
pub fn Field(props: FieldProps) -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 0.25rem;",
            label { style: "font-size: 0.8rem; color: #aaa;", "{props.label}" }
            {props.children}
        }
    }
}

#[component]
pub fn FieldGroup(children: Element) -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 0.8rem;", {children} }
    }
}
