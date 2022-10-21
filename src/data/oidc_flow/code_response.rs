use serde::{Deserialize, Serialize};

use super::oidc_scope::Scopes;

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeResponse {
    pub access_token: String,
    /// bearer
    pub token_type: String,
    pub id_token: String,
    pub scope: Scopes,
}
