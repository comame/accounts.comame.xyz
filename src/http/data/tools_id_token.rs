use serde::{Deserialize, Serialize};

use crate::data::oidc_flow::id_token_claim::IdTokenClaim;

#[derive(Serialize, Deserialize)]
pub struct IdTokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub id_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct IdTokenResponse {
    pub claim: IdTokenClaim,
    pub session: String,
}
