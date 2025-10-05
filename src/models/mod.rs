use std::time::{Duration, Instant};

use aes_gcm::{
    aead::{Aead, OsRng},
    AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
};
use argon2::Argon2;
use base64::prelude::*;
use zeroize::Zeroizing;

#[derive(Debug, Clone, PartialEq)]
pub struct PasswordEntrySafe {
    pub id: i32,
    pub site: String,
    pub username: String,
    pub password_hash: String,
}

impl PasswordEntrySafe {
    pub fn to_raw(&self, auth_state: &AuthState) -> Result<PasswordEntryRaw, String> {
        Ok(PasswordEntryRaw {
            id: self.id,
            site: self.site.clone(),
            username: self.username.clone(),
            raw_password: Zeroizing::new(self.decrypt_password(auth_state)?),
        })
    }

    fn decrypt_password(&self, auth_state: &AuthState) -> Result<String, String> {
        let key_bits = auth_state.get_key_material();
        let key = Key::<Aes256Gcm>::from_slice(&key_bits);
        let cipher = Aes256Gcm::new(key);

        let combined = BASE64_STANDARD
            .decode(self.password_hash.clone())
            .map_err(|err| err.to_string())?;

        let (nonce_bytes, ciphertext) = combined.split_at(12);

        let nonce: &Nonce<_> = Nonce::from_slice(nonce_bytes);

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|err| err.to_string())?;

        String::from_utf8(plaintext).map_err(|err| err.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PasswordEntryRaw {
    pub id: i32,
    pub site: String,
    pub username: String,
    pub raw_password: Zeroizing<String>,
}

impl PasswordEntryRaw {
    pub fn to_safe(&self, auth_state: &AuthState) -> Result<PasswordEntrySafe, String> {
        Ok(PasswordEntrySafe {
            id: self.id,
            site: self.site.clone(),
            username: self.username.clone(),
            password_hash: self.encrypt_password(auth_state)?,
        })
    }
    fn encrypt_password(&self, auth_state: &AuthState) -> Result<String, String> {
        let key_bits = auth_state.get_key_material();
        let key = Key::<Aes256Gcm>::from_slice(&key_bits);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, self.raw_password.as_bytes())
            .map_err(|err| err.to_string())?;

        // concatenate nonce + ciphertext
        let mut combined = nonce.to_vec();
        combined.extend_from_slice(&ciphertext);

        Ok(BASE64_STANDARD.encode(combined))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthState {
    pub signed_in: bool,
    pub raw_master_password: Zeroizing<String>,
    pub salt: Zeroizing<String>,
    pub last_activity: Instant,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            signed_in: false,
            raw_master_password: Zeroizing::new(String::new()),
            salt: Zeroizing::new(String::new()),
            last_activity: Instant::now(),
        }
    }
}

impl AuthState {
    pub fn get_key_material(&self) -> [u8; 32] {
        let mut output_key_material = [0u8; 32];
        Argon2::default()
            .hash_password_into(
                self.raw_master_password.as_bytes(),
                self.salt.as_bytes(),
                &mut output_key_material,
            )
            .unwrap();

        output_key_material
    }

    pub fn reset_idle_timer(&mut self) {
        self.last_activity = Instant::now();
    }

    pub fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.last_activity) > Duration::from_secs(3 * 60)
    }
}
