use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SessionSignInRequest {
    pub csrf_token: String,
    pub relying_party_id: String,
}
