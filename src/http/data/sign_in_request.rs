use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SignInRequest {
    pub user_id: String,
    pub password: String,
    pub csrf_token: String,
}
