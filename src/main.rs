use dioxus::{
    desktop::{Config, LogicalSize, WindowBuilder},
    prelude::*,
};

mod components;
mod routes;
mod views;

use routes::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const COMPONENT_CSS: Asset = asset!("/assets/styling/dx-components-theme.css");

fn main() {
    let launcher = init_launcher();
    launcher.launch(App);
}

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub signed_in: bool,
}

#[component]
fn App() -> Element {
    let state = use_signal(|| AppState::default());

    use_context_provider(move || state);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: COMPONENT_CSS }

        Router::<Route> {}
    }
}

fn init_launcher() -> LaunchBuilder {
    let app_name = "Password Manager";
    if cfg!(debug_assertions) {
        dioxus::LaunchBuilder::desktop().with_cfg(desktop! {
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title(app_name)
                    .with_inner_size(LogicalSize::new(800, 600))
            )
        })
    } else {
        dioxus::LaunchBuilder::desktop().with_cfg(desktop! {
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title(app_name)
                    .with_inner_size(LogicalSize::new(800, 600))
            ).with_menu(None)
        })
    }
}
