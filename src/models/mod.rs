use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PasswordEntrySafe {
    pub id: i32,
    pub site: String,
    pub username: String,
    pub password_hash: String,
}

impl PasswordEntrySafe {
    pub fn to_raw(&self, auth_state: &AuthState) -> PasswordEntryRaw {
        PasswordEntryRaw {
            id: self.id,
            site: self.site.clone(),
            username: self.username.clone(),
            raw_password: self.decrypt_password(auth_state),
        }
    }

    fn decrypt_password(&self, auth_state: &AuthState) -> String {
        // TODO: use key derivation function to decrypt the password.
        self.password_hash.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PasswordEntryRaw {
    pub id: i32,
    pub site: String,
    pub username: String,
    pub raw_password: String,
}

impl PasswordEntryRaw {
    pub fn to_safe(&self, auth_state: &AuthState) -> PasswordEntrySafe {
        PasswordEntrySafe {
            id: self.id,
            site: self.site.clone(),
            username: self.username.clone(),
            password_hash: self.encrypt_password(auth_state),
        }
    }
    fn encrypt_password(&self, auth_state: &AuthState) -> String {
        // TODO: use key derivation function to encrypt the password.
        self.raw_password.clone()
    }
}

// TODO: add key derivation function to the auth state and impl the default trait manually
#[derive(Debug, Clone, Default, PartialEq, Copy)]
pub struct AuthState {
    pub signed_in: bool,
}
