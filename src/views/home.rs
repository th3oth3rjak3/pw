use crate::{
    components::{Button, ButtonVariant},
    AppState,
};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let mut file_contents = use_signal(|| "".to_string());
    let mut state = use_context::<Signal<AppState>>();

    rsx! {
        div { {file_contents} }
        Button {
            variant: ButtonVariant::Secondary,
            onclick: move |_| {
                file_contents.set(read_file_contents());
                state.set(AppState { signed_in: true });
            },
            "Click Me"
        }
    }
}

fn read_file_contents() -> String {
    std::fs::read_to_string("/home/th3oth3rjak3/.password_manager/thing.txt").unwrap()
}
