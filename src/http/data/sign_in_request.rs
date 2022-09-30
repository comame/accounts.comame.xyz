use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SignInRequest {
    pub user_id: String,
    pub password: String,
    pub csrf_token: String,
}
