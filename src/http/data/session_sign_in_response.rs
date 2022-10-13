use serde::Serialize;

#[derive(Serialize)]
pub struct SessionSignInResponse {
    pub user_id: String,
    pub last_auth: Option<u64>,
}
