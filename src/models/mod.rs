use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub id: i32,
    pub site: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Default, PartialEq, Copy)]
pub struct AuthState {
    pub signed_in: bool,
}
