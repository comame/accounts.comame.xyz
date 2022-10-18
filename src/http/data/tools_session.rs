use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SessionInspectRequest {
    pub client_id: String,
    pub client_secret: String,
    pub token: String,
}

#[derive(Serialize)]
pub struct SessionInspectResponse {
    pub user_id: String,
}
