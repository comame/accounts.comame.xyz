use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeResponse {
    pub access_token: String,
    /// bearer
    pub token_type: String,
    pub id_token: String,
    pub scope: String,
}
