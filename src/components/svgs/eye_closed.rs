use dioxus::prelude::*;

#[component]
pub fn EyeClosed() -> Element {
    rsx! {
        svg {
            width: "24",
            "viewBox": "0 0 24 24",
            stroke: "currentColor",
            "stroke-width": "2",
            height: "24",
            fill: "none",
            "stroke-linecap": "round",
            xmlns: "http://www.w3.org/2000/svg",
            "stroke-linejoin": "round",
            class: "lucide lucide-eye-closed-icon lucide-eye-closed",
            path { d: "m15 18-.722-3.25" }
            path { d: "M2 8a10.645 10.645 0 0 0 20 0" }
            path { d: "m20 15-1.726-2.05" }
            path { d: "m4 15 1.726-2.05" }
            path { d: "m9 18 .722-3.25" }
        }
    }
}
