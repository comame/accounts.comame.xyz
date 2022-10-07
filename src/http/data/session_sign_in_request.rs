use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SessionSignInRequest {
    pub csrf_token: String,
}
