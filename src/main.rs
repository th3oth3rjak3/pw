use std::{
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use dioxus::{
    desktop::{Config, LogicalSize, WindowBuilder},
    prelude::*,
};

mod components;
mod models;
mod routes;
mod services;
mod views;

use directories::UserDirs;
use routes::Route;

use crate::{components::ToastProvider, models::AuthState, services::database::DatabaseService};

static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

fn main() {
    init_data_directory();

    let launcher = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move { init_launcher().await });

    launcher.launch(App);
}

#[component]
fn App() -> Element {
    provide_context(Signal::new(AuthState::default()));

    let main_css = include_str!("../assets/styling/main.css");
    let dx_component_theme = include_str!("../assets/styling/dx-components-theme.css");

    rsx! {
        style { {main_css} }
        style { {dx_component_theme} }

        ToastProvider { Router::<Route> {} }
    }
}

async fn init_launcher() -> LaunchBuilder {
    let app_name = "Password Manager";

    let db_service =
        Arc::new(DatabaseService::new(DATA_DIR.get().unwrap().join("passwords.sqlite")).await);

    dioxus::LaunchBuilder::desktop()
        .with_cfg(desktop! {
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title(app_name)
                    .with_min_inner_size(LogicalSize::new(600, 600))
                    .with_inner_size(LogicalSize::new(800, 600))
            )
            .with_menu(None)
        })
        .with_context(db_service)
}

fn init_data_directory() {
    if let Some(user_dirs) = UserDirs::new() {
        let home_dir = user_dirs.home_dir();
        let data_dir = home_dir.join(".password_manager");

        std::fs::create_dir_all(&data_dir).expect("could not create application data directory");
        DATA_DIR.set(data_dir.clone()).unwrap();
    } else {
        println!("Could not find user directories. Exiting...");
        std::process::exit(1);
    }
}
