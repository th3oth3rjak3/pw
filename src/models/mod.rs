use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Password {
    pub site: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PasswordData {
    pub master_password_hash: String,
    pub passwords: Vec<Password>,
}
