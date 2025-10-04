use std::sync::Arc;

use crate::{
    routes::Route,
    services::{
        authentication::{self},
        database::DatabaseService,
    },
};
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let db_service = use_context::<Arc<DatabaseService>>();
    let navigator = use_navigator();
    let toast_api = use_toast();

    spawn(async move {
        match authentication::is_master_password_set(&db_service).await {
            Ok(is_set) => {
                if is_set {
                    navigator.replace(Route::login());
                } else {
                    navigator.replace(Route::create_master_password());
                }
            }
            Err(err) => toast_api.error(
                "Critical Error".into(), 
                ToastOptions::new()
                    .description(format!("An error occurred while trying to verify your master password was set: {err}"))
                    .permanent(true)),
        }
    });

    rsx! {}
}
