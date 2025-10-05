use dioxus::prelude::*;
use dioxus_primitives::navbar::{
    self, NavbarContentProps, NavbarItemProps, NavbarNavProps, NavbarProps, NavbarTriggerProps,
};

use crate::models::AuthState;

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    let style = include_str!("./style.css");

    rsx! {
        style { {style} }

        navbar::Navbar {
            class: "navbar",
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarNav(props: NavbarNavProps) -> Element {
    rsx! {
        navbar::NavbarNav {
            class: "navbar-nav",
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarTrigger(props: NavbarTriggerProps) -> Element {
    rsx! {
        navbar::NavbarTrigger { class: "navbar-trigger", attributes: props.attributes,
            {props.children}
            svg {
                class: "navbar-expand-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn NavbarContent(props: NavbarContentProps) -> Element {
    rsx! {
        navbar::NavbarContent {
            class: "navbar-content",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarItem(props: NavbarItemProps) -> Element {
    let mut state = use_context::<Signal<AuthState>>();
    rsx! {
        navbar::NavbarItem {
            class: "navbar-item",
            index: props.index,
            value: props.value,
            disabled: props.disabled,
            new_tab: props.new_tab,
            to: props.to,
            active_class: props.active_class,
            attributes: props.attributes,
            on_select: props.on_select,
            onclick: move |evt| {
                state.write().reset_idle_timer();
                if let Some(func) = props.onclick {
                    func.call(evt);
                }
            },
            onmounted: props.onmounted,
            {props.children}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct NavbarButtonProps {
    /// Whether this nav item is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    pub children: Element,
    #[props(optional)]
    pub onclick: Callback<MouseEvent>,
    /// Additional attributes to apply to the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The [`NavbarButton`] is used to perform actions on the navbar that don't actually navigate
/// to another location.
#[component]
pub fn NavbarButton(props: NavbarButtonProps) -> Element {
    let mut state = use_context::<Signal<AuthState>>();
    let disabled = move || (props.disabled)();

    rsx! {
        a {
            class: "navbar-item",
            "data-disabled": disabled(),
            onclick: move |evt| {
                state.write().reset_idle_timer();
                props.onclick.call(evt);
            },
            ..props.attributes,
            {props.children}

        }
    }
}
