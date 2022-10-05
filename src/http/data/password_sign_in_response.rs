use serde::Serialize;

#[derive(Serialize)]
pub struct PasswordSignInResponse {
    user_id: String,
}

impl PasswordSignInResponse {
    pub fn new(user_id: &str) -> Self {
        PasswordSignInResponse {
            user_id: user_id.to_string(),
        }
    }
}
