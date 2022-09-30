use serde::Serialize;

#[derive(Serialize)]
pub struct SignInResponse {
    user_id: String,
}

impl SignInResponse {
    pub fn new(user_id: &str) -> SignInResponse {
        SignInResponse { user_id: user_id.to_string() }
    }
}
