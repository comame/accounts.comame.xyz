use serde::Serialize;

use crate::data::oidc_relying_party::RelyingParty;

#[derive(Serialize)]
pub struct RelyingPartyRawSecretResponse {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize)]
pub struct RelyingPartiesResponse {
    pub values: Vec<RelyingParty>
}
