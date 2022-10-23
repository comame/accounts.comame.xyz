use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserIdRequest {
    pub token: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct UserIdPasswordRequest {
    pub token: String,
    pub user_id: String,
    pub password: String,
}
