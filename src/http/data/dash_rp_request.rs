use serde::Deserialize;

#[derive(Deserialize)]
pub struct RelyingPartyClientIdRequest {
    pub token: String,
    pub client_id: String,
}
