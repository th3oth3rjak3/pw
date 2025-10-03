use crate::views::*;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},

        #[route("/create_master_password")]
        CreateMasterPassword {},

        #[route("/vault")]
        Vault {}
}

impl Route {
    pub fn vault() -> Self {
        Route::Vault {}
    }

    pub fn home() -> Self {
        Route::Home {}
    }
}
