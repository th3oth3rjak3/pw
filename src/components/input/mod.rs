use dioxus::prelude::*;

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
