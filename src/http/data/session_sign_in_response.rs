use serde::Serialize;

#[derive(Serialize)]
pub struct SessionSignInResponse {
    pub user_id: String,
}
