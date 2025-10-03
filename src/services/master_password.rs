use crate::AppState;

pub fn verify_master_password(raw_pw: Vec<u8>, state: &AppState) -> Result<bool, anyhow::Error> {
    Ok(true)
}

pub fn reset_master_password(
    old_raw_pw: Vec<u8>,
    new_raw_pw: Vec<u8>,
    state: &mut AppState,
) -> anyhow::Result<bool> {
    todo!()
}
