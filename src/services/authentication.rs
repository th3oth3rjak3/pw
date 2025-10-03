use crate::AppState;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoginError {
    IncorrectPassword,
}

pub fn login(raw_pw: Vec<u8>, mut state: AppState) -> Result<AppState, LoginError> {
    // TODO: Implement the real login behavior like validating the password hash
    if raw_pw == "test".as_bytes().to_vec() {
        // TODO: set the expiry time.
        state.signed_in = true;
        Ok(state)
    } else {
        Err(LoginError::IncorrectPassword)
    }
}

pub fn logout() -> AppState {
    AppState::default()
}
