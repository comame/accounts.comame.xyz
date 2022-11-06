use serde::Serialize;

use crate::data::{oidc_relying_party::RelyingParty, user_binding::UserBinding};

#[derive(Serialize)]
pub struct RelyingPartyRawSecretResponse {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize)]
pub struct RelyingPartiesResponse {
    pub values: Vec<RelyingParty>,
}

#[derive(Serialize)]
pub struct RelyingPartyBindingResponse {
    pub values: Vec<UserBinding>,
}
