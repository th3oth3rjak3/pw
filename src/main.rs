use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
    sync::OnceLock,
};

use dioxus::{
    desktop::{Config, LogicalSize, WindowBuilder},
    prelude::*,
};

mod components;
mod models;
mod routes;
mod views;

use directories::UserDirs;
use routes::Route;

use crate::models::PasswordData;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const COMPONENT_CSS: Asset = asset!("/assets/styling/dx-components-theme.css");

static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

fn main() {
    init_data_directory();

    let launcher = init_launcher();
    launcher.launch(|| {
        let data = load_password_data();
        let props = AppProps {
            state: AppState::new(false, data),
        };

        App(props)
    });
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AppState {
    pub signed_in: bool,
    pub password_data: PasswordData,
}

impl AppState {
    pub fn new(signed_in: bool, password_data: PasswordData) -> Self {
        Self {
            signed_in,
            password_data,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
struct AppProps {
    state: AppState,
}

#[component]
fn App(props: AppProps) -> Element {
    let state = use_signal(|| props.state);

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

fn load_password_data() -> PasswordData {
    let password_file_path = DATA_DIR.get().unwrap().join("passwords.json");

    if password_file_path.exists() {
        // do something with the file like load its contents
        let mut password_file = fs::File::create_new(password_file_path)
            .unwrap_or_else(|_| bail("Could not create required password file."));

        let mut file_contents = String::new();
        password_file
            .read_to_string(&mut file_contents)
            .unwrap_or_else(|_| bail("Could not read password file contents."));

        let password_data: PasswordData = serde_json::from_str(&file_contents)
            .unwrap_or_else(|_| bail("Could not read password file contents."));

        return password_data;
    } else {
        // create the file

        let mut password_file = fs::File::create_new(password_file_path)
            .unwrap_or_else(|_| bail("Could not create required password file."));

        let data = serde_json::to_string(&PasswordData::default())
            .unwrap_or_else(|_| bail("Can not generate password contents."));

        password_file
            .write_all(data.as_bytes())
            .unwrap_or_else(|_| bail("Can not write to password file."));

        PasswordData::default()
    }
}

fn bail(error: impl Into<String>) -> ! {
    let message: String = error.into();
    eprintln!("{message}");
    std::process::exit(1);
}
