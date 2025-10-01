use dioxus::prelude::*;

use crate::components::{Button, ButtonVariant};

#[component]
pub fn CreateMasterPassword() -> Element {
    rsx! {
        h3 { "Create Master Password" }
    }
}
