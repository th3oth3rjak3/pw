use std::time::Instant;

use crate::{
    models::AuthState,
    services::{database::DatabaseService, password_entry},
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use sqlx::prelude::*;
use sqlx::SqlitePool;
use zeroize::Zeroizing;

#[derive(Debug, Clone, PartialEq)]
pub enum LoginError {
    IncorrectPassword,
    HashingError(String),
}

pub async fn login(
    raw_pw: Zeroizing<String>,
    mut state: AuthState,
    db_service: &DatabaseService,
) -> Result<AuthState, LoginError> {
    let (saved_hash, key_derivation_salt) = get_master_password_hash(&db_service.pool)
        .await
        .expect("Could not get master password hash from the database");

    let parsed_hash = PasswordHash::new(&saved_hash)
        .map_err(|err| err.to_string())
        .map_err(LoginError::HashingError)?;

    if Argon2::default()
        .verify_password(raw_pw.as_bytes(), &parsed_hash)
        .is_ok()
    {
        state.signed_in = true;
        state.raw_master_password = raw_pw;
        state.salt = Zeroizing::new(key_derivation_salt);
        state.last_activity = Instant::now();
        Ok(state)
    } else {
        Err(LoginError::IncorrectPassword)
    }
}

pub async fn set_master_password(
    raw_pw: Zeroizing<String>,
    auth_state: &AuthState,
    db_service: &DatabaseService,
) -> Result<AuthState, String> {
    let existing_password_entries =
        password_entry::get_all_password_entries(auth_state, db_service, String::new()).await?;

    let hash = hash_new_master_password(&raw_pw).map_err(|err| err.to_string())?;
    let salt = SaltString::generate(&mut OsRng).to_string();

    let mut tx = db_service
        .pool
        .begin()
        .await
        .map_err(|err| err.to_string())?;

    sqlx::query("update master_password set password_hash = ?, key_salt = ? where id = 1;")
        .bind(hash)
        .bind(salt.clone())
        .execute(&mut *tx)
        .await
        .map(|_| ())
        .map_err(|err| err.to_string())?;

    let new_auth_state = AuthState {
        signed_in: true,
        raw_master_password: raw_pw,
        salt: Zeroizing::new(salt),
        last_activity: Instant::now(),
    };

    for entry in existing_password_entries.iter() {
        password_entry::save_updated_password(
            entry.id,
            entry.to_owned(),
            &new_auth_state,
            &mut *tx,
        )
        .await?;
    }

    tx.commit()
        .await
        .map(|_| new_auth_state)
        .map_err(|err| err.to_string())
}

pub fn logout() -> AuthState {
    AuthState::default()
}

pub async fn is_master_password_set(db_service: &DatabaseService) -> Result<bool, String> {
    let (hash, _) = get_master_password_hash(&db_service.pool)
        .await
        .map_err(|err| err.to_string())?;

    Ok(!hash.is_empty())
}

async fn get_master_password_hash(pool: &SqlitePool) -> Result<(String, String), sqlx::Error> {
    let row = sqlx::query("SELECT password_hash, key_salt FROM master_password WHERE id = 1")
        .fetch_one(pool)
        .await?;

    Ok((row.get("password_hash"), row.get("key_salt")))
}

fn hash_new_master_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| err.to_string())?
        .to_string();

    Ok(password_hash)
}
