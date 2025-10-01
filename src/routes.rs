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
