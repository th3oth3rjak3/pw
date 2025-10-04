use dioxus::core::spawn;
use sqlx::{prelude::*, Database};

use crate::{
    models::{AuthState, PasswordEntryRaw, PasswordEntrySafe},
    services::database::DatabaseService,
};

pub fn create_password_entry() -> anyhow::Result<AuthState> {
    todo!()
}

pub async fn get_all_password_entries(
    auth_state: &AuthState,
    db_service: &DatabaseService,
) -> Result<Vec<PasswordEntryRaw>, String> {
    if !auth_state.signed_in {
        return Err("You must be signed in to access these resources".into());
    }

    let rows = sqlx::query("select id, site, username, password_hash from password_entries")
        .fetch_all(&db_service.pool)
        .await
        .map_err(|err| err.to_string())?;

    let mut password_entries: Vec<PasswordEntrySafe> = Vec::with_capacity(rows.len());

    for row in rows {
        let id: i32 = row.get("id");
        let site: String = row.get("site");
        let username: String = row.get("username");
        let password_hash: String = row.get("password_hash");

        password_entries.push(PasswordEntrySafe {
            id,
            site,
            username,
            password_hash,
        });
    }

    Ok(password_entries
        .into_iter()
        .map(|safe_entry| safe_entry.to_raw(auth_state))
        .collect::<Vec<_>>())
}

pub async fn get_password_entry_by_id(
    id: i32,
    auth_state: &AuthState,
    db_service: &DatabaseService,
) -> Result<PasswordEntryRaw, String> {
    if !auth_state.signed_in {
        return Err("You must be signed in to access these resources".into());
    }

    let row =
        sqlx::query("select id, site, username, password_hash from password_entries where id = ?;")
            .bind(id)
            .fetch_one(&db_service.pool)
            .await
            .map_err(|err| err.to_string())?;

    let safe_entry = PasswordEntrySafe {
        id: row.get("id"),
        site: row.get("site"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
    };

    Ok(safe_entry.to_raw(auth_state))
}

pub async fn save_updated_password(
    id: i32,
    password_entry: PasswordEntryRaw,
    auth_state: &AuthState,
    db_service: &DatabaseService,
) -> Result<(), String> {
    todo!()
}
