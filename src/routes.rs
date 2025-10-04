use crate::views::*;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},

        #[route("/login")]
        Login {},

        #[route("/create_master_password")]
        CreateMasterPassword {},

        #[route("/vault")]
        Vault {},

        #[route("/vault/:id")]
        PasswordDetails { id: i32 },

        #[route("/new_password_entry")]
        NewPasswordEntry {}
}

impl Route {
    pub fn vault() -> Self {
        Route::Vault {}
    }

    pub fn home() -> Self {
        Route::Home {}
    }

    pub fn login() -> Self {
        Route::Login {}
    }

    pub fn create_master_password() -> Self {
        Route::CreateMasterPassword {}
    }

    pub fn password_details(id: i32) -> Self {
        Route::PasswordDetails { id }
    }

    pub fn new_password_entry() -> Self {
        Route::NewPasswordEntry {}
    }
}
