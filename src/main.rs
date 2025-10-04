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

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const COMPONENT_CSS: Asset = asset!("/assets/styling/dx-components-theme.css");

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

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: COMPONENT_CSS }

        ToastProvider { Router::<Route> {} }
    }
}

async fn init_launcher() -> LaunchBuilder {
    let app_name = "Password Manager";

    let db_service =
        Arc::new(DatabaseService::new(DATA_DIR.get().unwrap().join("passwords.sqlite")).await);

    let mut builder = dioxus::LaunchBuilder::desktop();

    if cfg!(debug_assertions) {
        builder = builder.with_cfg(desktop! {
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title(app_name)
                    .with_inner_size(LogicalSize::new(800, 600))
            )
        })
    } else {
        builder = builder.with_cfg(desktop! {
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title(app_name)
                    .with_inner_size(LogicalSize::new(800, 600))
            ).with_menu(None)
        })
    }

    builder = builder.with_context(db_service);

    builder
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
