use dioxus::prelude::*;

/// A reusable card with a title and arbitrary content.
#[derive(Debug, Clone, Props, PartialEq)]
pub struct CardProps {
    pub title: String,
    #[props(extends=GlobalAttributes)]
    attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div {
            style: "
                background: #1e1e1e;
                padding: 1rem 2rem;
                border-radius: 16px;
                width: 100%;
                box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
                border: 1px solid #2a2a2a;
            ",
            ..props.attributes,

            if !props.title.is_empty() {
                h2 { style: "
                        font-weight: 600;
                        text-align: center;
                        color: #f0f0f0;
                    ",
                    "{props.title}"
                }
            }
            div { style: "display: flex; flex-direction: column; gap: 0.5rem;", {props.children} }
        }
    }
}
