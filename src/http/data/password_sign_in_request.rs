use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PasswordSignInRequest {
    pub user_id: String,
    pub password: String,
    pub csrf_token: String,
    pub relying_party_id: String,
}
